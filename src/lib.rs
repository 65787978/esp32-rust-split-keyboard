/*
PINS                                                PINS
___|  0  |  1  |  12 |  18 |  19 |  13 |            ___|  0  |  1  |  12 |  18 |  19 |  13 |
 2 |_ESC_|__1__|__2__|__3__|__4__|__5__|             2 |__6__|__7__|__8__|__9__|__0__|__-__|
 3 |_TAB_|__Q__|__W__|__E__|__R__|__T__|             3 |__Y__|__U__|__I__|__O__|__P__|__{__|
10 |_CAP_|__A__|__S__|__D__|__F__|__G__|             10|__H__|__J__|__K__|__L__|__;__|__}__|
 6 |_SFT_|__Z__|__X__|__C__|__V__|__B__|             6 |__N__|__M__|__,__|__.__|__/__|__\__|
 7 |_____|_____|_____|_CTL_|_BSP_|_DEL_|             7 |_CTL_|_ENT_|_SPC_|_FUN_|_____|_____|

*/
use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
pub struct KeyboardLeftSide {
    pub key: HashMap<(i32, i32), &'static str>,
}

impl KeyboardLeftSide {
    pub fn new() -> KeyboardLeftSide {
        KeyboardLeftSide {
            key: HashMap::new(),
        }
    }

    pub fn initialize_hashmap(&mut self) {
        self.key.insert((2, 0), "ESC"); /* ESC */
        self.key.insert((2, 1), "1"); /* 1 */
        self.key.insert((2, 12), "2"); /* 2 */
        self.key.insert((2, 18), "3"); /* 3 */
        self.key.insert((2, 19), "4"); /* 4 */
        self.key.insert((2, 13), "5"); /* 5 */

        self.key.insert((3, 0), "TAB"); /* TAB */
        self.key.insert((3, 1), "Q"); /* Q */
        self.key.insert((3, 12), "W"); /* W */
        self.key.insert((3, 18), "E"); /* E */
        self.key.insert((3, 19), "R"); /* R */
        self.key.insert((3, 13), "T"); /* T */

        self.key.insert((10, 0), "CAP"); /* CAP */
        self.key.insert((10, 1), "A"); /* A */
        self.key.insert((10, 12), "S"); /* S */
        self.key.insert((10, 18), "D"); /* D */
        self.key.insert((10, 19), "F"); /* F */
        self.key.insert((10, 13), "G"); /* G */

        self.key.insert((6, 0), "SFT"); /* SFT */
        self.key.insert((6, 1), "Z"); /* Z */
        self.key.insert((6, 12), "X"); /* X */
        self.key.insert((6, 18), "C"); /* C */
        self.key.insert((6, 19), "V"); /* V */
        self.key.insert((6, 13), "B"); /* B */

        self.key.insert((7, 0), "PLACEHOLDER"); /* placeHolder */
        self.key.insert((7, 1), "PLACEHOLDER"); /* placeHolder */
        self.key.insert((7, 12), "PLACEHOLDER"); /* placeHolder */
        self.key.insert((7, 18), "CTL"); /* CTL */
        self.key.insert((7, 19), "BSP"); /* BSP */
        self.key.insert((7, 13), "DEL"); /* DEL */
    }
}
