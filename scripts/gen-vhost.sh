#!/usr/bin/env bash
# KUSANAGIの「サイト追加」相当: ドメイン/サブドメイン + IP + バックエンドの
# 組み合わせから Nginx/Apache の vhost 設定ファイル(HTTP+HTTPS)を自動生成する。
# 実際のDNS登録(レジストラでのAレコード/CNAME追加)は別途手動で行うこと。
# TLS証明書の取得・自動更新は scripts/setup-tls.sh を参照。
#
# 使い方:
#   scripts/gen-vhost.sh <DOMAIN> <BIND_IP> <UPSTREAM_HOST:PORT> [WEBROOT]
#
# 例(aruaru-web本体):
#   scripts/gen-vhost.sh aruaru.example.com 203.0.113.10 127.0.0.1:4000
# 例(サブドメインで別用途:例えば社内ツール):
#   scripts/gen-vhost.sh tool.example.com 203.0.113.10 127.0.0.1:9000 /var/www/tool

set -euo pipefail

if [ $# -lt 3 ]; then
  echo "使い方: $0 <DOMAIN> <BIND_IP> <UPSTREAM_HOST:PORT> [WEBROOT]" >&2
  exit 1
fi

DOMAIN="$1"
BIND_IP="$2"
UPSTREAM="$3"
WEBROOT="${4:-/var/www/${DOMAIN}}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
OUT_DIR="${REPO_ROOT}/deploy/generated"
mkdir -p "${OUT_DIR}"

render() {
  local template="$1" out="$2"
  sed \
    -e "s#{{DOMAIN}}#${DOMAIN}#g" \
    -e "s#{{IP}}#${BIND_IP}#g" \
    -e "s#{{UPSTREAM}}#${UPSTREAM}#g" \
    -e "s#{{WEBROOT}}#${WEBROOT}#g" \
    "${template}" > "${out}"
  echo "生成: ${out}"
}

render "${REPO_ROOT}/deploy/nginx/vhost.conf.template"  "${OUT_DIR}/${DOMAIN}.nginx.conf"
render "${REPO_ROOT}/deploy/apache/vhost.conf.template" "${OUT_DIR}/${DOMAIN}.apache.conf"

# 監視/自動更新対象ドメインの一覧に追記(scripts/check-all-tls.sh が参照)。
DOMAINS_FILE="${OUT_DIR}/domains.txt"
touch "${DOMAINS_FILE}"
if ! grep -qxF "${DOMAIN}" "${DOMAINS_FILE}"; then
  echo "${DOMAIN}" >> "${DOMAINS_FILE}"
fi

cat <<EOF

次の手順:
  1. レジストリでドメイン/サブドメイン(${DOMAIN})のAレコードを ${BIND_IP} に向ける(DNS登録は各自実施)。
  2. 生成された設定ファイルを Nginx/Apache の設定ディレクトリに配置し、リロードする。
     - Nginx : ${OUT_DIR}/${DOMAIN}.nginx.conf
     - Apache: ${OUT_DIR}/${DOMAIN}.apache.conf
  3. HTTPS証明書を取得・自動更新を有効化する:
       scripts/setup-tls.sh ${DOMAIN} admin@${DOMAIN} ${WEBROOT}
  4. 有効期限の自動監視を有効化する(未設定なら1回だけ):
       deploy/systemd/install-systemd-units.sh
  5. 複数サイトを管理する場合は、このスクリプトをドメインごとに繰り返し実行する
     (aruaru-web用、別プロジェクト用など、UPSTREAMとWEBROOTを変えるだけでよい)。
  6. aruaru-web の GUI 側「サイト管理」タブでも同じ接続先情報を登録しておくと、
     ブラウザ側から接続先を切り替えられる。
EOF
