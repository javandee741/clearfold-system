from pathlib import Path
import json, sys
root=Path(__file__).resolve().parents[1]
status=json.loads((root/'docs/translation-status.json').read_text(encoding='utf-8'))
allowed={'complete','planned','stale','in-progress'}
errors=[]
for doc,langs in status['documents'].items():
    for lang,state in langs.items():
        if state not in allowed:
            errors.append(f'{doc}:{lang} invalid state {state}')
if errors:
    print('\n'.join(errors)); sys.exit(1)
print('Translation metadata OK')
