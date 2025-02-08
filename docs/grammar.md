D -> HRT | RT | T
T -> BF

/* Headers */
H -> Ht | Hs
Ht -> '==' epsilon
Hs -> '--' epsilon

/* Blocks */
B -> Bq | Bc
Bq -> '> ' T
Bc -> '```\n' epsilon '```'

/* Formatting */
F -> Fb | Fi | Fs | epsilon
Fb -> '{' F '}'
Fi -> '_' F '_'
Fs -> '~' F '~'
