JSON=`cat landing.json`
curl -i -H "Accept: application/json" -H "Content-Type: application/json" -H "x-client-address: 0.0.0.0" -X POST -d "$JSON" http://localhost:5555/analytics/landing -v