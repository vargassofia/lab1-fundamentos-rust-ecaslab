#!/usr/bin/env bash
# Comprueba que cada archivo de esta carpeta falle al compilar
# con el error que indica su encabezado ("// Error esperado: ...").
set -u
cd "$(dirname "$0")"
salida=$(mktemp -d)
fallos=0
for archivo in e*.rs; do
    esperado=$(sed -n 's|^// Error esperado: ||p' "$archivo")
    if mensaje=$(rustc --edition 2024 --out-dir "$salida" "$archivo" 2>&1); then
        echo "FALLA  $archivo compiló, pero debía dar $esperado"
        fallos=$((fallos + 1))
    elif grep -qF -- "$esperado" <<<"$mensaje"; then
        echo "ok     $archivo -> $esperado"
    else
        echo "FALLA  $archivo no produjo $esperado"
        fallos=$((fallos + 1))
    fi
done
rm -rf "$salida"
exit "$fallos"
