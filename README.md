# sware-server

## SN Loader
Loads events from Spotter Network's reports page.

### TODO
- Update when new API goes live. Keep checking, since it supposedly already did?

## NWS Loader
Loads a selected set of events from the NWS API (api.weather.gov).

### Implemented products
Details of product codes and products can be found at: https://en.wikipedia.org/wiki/Specific_Area_Message_Encoding
- `AFD` Area Forecast Discussion
- `LSR` Local Storm Report
- `SEL` Severe Local Storm Watch and Watch Cancellation Msg. Issued when watches are issued. Has the watch text.
- `SVR` Severe Thunderstorm Warning
- `SVS` Severe Weather Statement (only PDS and tornado emergency)
- `SWO` Severe Storm Outlook Narrative. Includes the 1/2/3/4-8 day outlooks (ACUS01/02/03/48) and Mesoscale Discussions (ACUS11). MDs contain their own coordinates and do not have a corresponding PTS.
- `TOR` Tornado Warning
- `FFW` Flash Flood Warning

### Missing products (that should be implemented in order of priority)
- `SEV` Shows coordinates for all active watches.
- `PTS` Probabilistic Outlook Points. Contains coordinates for SWO outlooks (WUUS01/02/03/48).
- `FFA` Flash Flood Watch (need sample)

### TODO
- get location/direction for PDS TORs in SVS: 1587342012426610
- should not set off alert for canceling PDS TOR warning in SVS: 1587343271629732
- handle flash floods in LSR (data/products/lsr-flashflood)
- handle multiple events in an LSR
- check on TSTM and non-severe outlooks once they happen, to finish get_outlook_risk
- implement sev/pts once mapping client exists

# TODO
- nginx service not working right
- deploy spa
- backup nginx configs
- add faster polling times when certain events seen, ie. more TOR/LSR/SN polling when in a tornado watch
- increase test coverage
- expose store stats via route
- add benchmarks
- remove all unwraps or look into parser combinators
