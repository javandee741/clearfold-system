from pathlib import Path
import json, sys
root = Path(__file__).resolve().parents[1]
required = [
    'README.md','LICENSE','SECURITY.md','CONTRIBUTING.md','ROADMAP.md',
    'spec/kernel-objects.md','spec/syscall-abi.md','docs/TRANSLATIONS.md',
    'site/index.html'
]
missing=[p for p in required if not (root/p).exists()]
if missing:
    print('Missing required files:', *missing, sep='\n - ')
    sys.exit(1)
status=json.loads((root/'docs/translation-status.json').read_text(encoding='utf-8'))
assert status['baseline']=='0.15'
print('Repository structure OK')
