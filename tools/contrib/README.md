# Contribution board generator

Renders GitHub's own contribution counts as a split-flap style board (`assets/contributions.png`).
One tile per real day. Shade is the day's count relative to the busiest day, in four steps. No values are invented.

```sh
# 1. export real counts (counts and dates only)
gh api graphql -f query='query{viewer{contributionsCollection{contributionCalendar{totalContributions weeks{contributionDays{date contributionCount weekday}}}}}}' > /tmp/cc.json
python3 - <<'PY'
import json
d=json.load(open('/tmp/cc.json'))['data']['viewer']['contributionsCollection']['contributionCalendar']
print('week,weekday,date,count')
for i,w in enumerate(d['weeks']):
    for x in w['contributionDays']:
        print(f"{i},{x['weekday']},{x['date']},{x['contributionCount']}")
print('API total', d['totalContributions'], file=__import__('sys').stderr)
PY > data/contributions.csv

# 2. render
cd tools/contrib && cargo run --release -- --data ../../data/contributions.csv --out ../../assets/contributions.png
```

The printed total must equal the API total. Run locally when you want a refresh; there is no workflow, so no CI cost.
GitHub's native graph on the profile is the live record and cannot be restyled.
