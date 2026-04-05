//! Generated from ukrainian.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use snowball::SnowballEnv;
use snowball::Among;

#[derive(Clone)]
struct Context {
    i_p2: i32,
    i_pV: i32,
}

static A_0: &'static [Among<Context>; 4] = &[
    Among("вшись", -1, 1, None),
    Among("ившись", 0, 2, None),
    Among("вши", -1, 1, None),
    Among("ивши", 2, 2, None),
];

static A_1: &'static [Among<Context>; 7] = &[
    Among("сь", -1, 1, None),
    Among("лась", 0, 1, None),
    Among("всь", 0, 1, None),
    Among("тись", 0, 1, None),
    Among("ся", -1, 1, None),
    Among("вся", 4, 1, None),
    Among("тися", 4, 1, None),
];

static A_2: &'static [Among<Context>; 128] = &[
    Among("ому", -1, 2, None),
    Among("ььому", 0, 2, None),
    Among("ічному", 0, 1, None),
    Among("ичному", 0, 1, None),
    Among("уальному", 0, 1, None),
    Among("ійному", 0, 1, None),
    Among("аційному", 5, 1, None),
    Among("онному", 0, 1, None),
    Among("аціях", -1, 1, None),
    Among("іх", -1, 2, None),
    Among("їх", -1, 2, None),
    Among("их", -1, 2, None),
    Among("ічних", 11, 1, None),
    Among("ичних", 11, 1, None),
    Among("уальних", 11, 1, None),
    Among("ійних", 11, 1, None),
    Among("аційних", 15, 1, None),
    Among("онних", 11, 1, None),
    Among("єю", -1, 2, None),
    Among("ацією", 18, 1, None),
    Among("ацію", -1, 1, None),
    Among("ою", -1, 2, None),
    Among("ььою", 21, 2, None),
    Among("ічною", 21, 1, None),
    Among("ичною", 21, 1, None),
    Among("уальною", 21, 1, None),
    Among("ійною", 21, 1, None),
    Among("аційною", 26, 1, None),
    Among("онною", 21, 1, None),
    Among("ація", -1, 1, None),
    Among("учі", -1, 2, None),
    Among("ючі", -1, 2, None),
    Among("ічні", -1, 1, None),
    Among("ичні", -1, 1, None),
    Among("уальні", -1, 1, None),
    Among("увані", -1, 2, None),
    Among("овані", -1, 2, None),
    Among("ійні", -1, 1, None),
    Among("аційні", 37, 1, None),
    Among("онні", -1, 1, None),
    Among("оі", -1, 2, None),
    Among("ььоі", 40, 2, None),
    Among("ічноі", 40, 1, None),
    Among("ичноі", 40, 1, None),
    Among("уальноі", 40, 1, None),
    Among("ійноі", 40, 1, None),
    Among("аційноі", 45, 1, None),
    Among("онноі", 40, 1, None),
    Among("ації", -1, 1, None),
    Among("ої", -1, 2, None),
    Among("ььої", 49, 2, None),
    Among("ічної", 49, 1, None),
    Among("ичної", 49, 1, None),
    Among("уальної", 49, 1, None),
    Among("ійної", 49, 1, None),
    Among("аційної", 54, 1, None),
    Among("онної", 49, 1, None),
    Among("уча", -1, 2, None),
    Among("юча", -1, 2, None),
    Among("ічна", -1, 1, None),
    Among("ична", -1, 1, None),
    Among("уальна", -1, 1, None),
    Among("увана", -1, 2, None),
    Among("ована", -1, 2, None),
    Among("ійна", -1, 1, None),
    Among("аційна", 64, 1, None),
    Among("онна", -1, 1, None),
    Among("уче", -1, 2, None),
    Among("юче", -1, 2, None),
    Among("ічне", -1, 1, None),
    Among("ичне", -1, 1, None),
    Among("уальне", -1, 1, None),
    Among("уване", -1, 2, None),
    Among("оване", -1, 2, None),
    Among("ійне", -1, 1, None),
    Among("аційне", 74, 1, None),
    Among("онне", -1, 1, None),
    Among("аціями", -1, 1, None),
    Among("іми", -1, 2, None),
    Among("їми", -1, 2, None),
    Among("ими", -1, 2, None),
    Among("ічними", 80, 1, None),
    Among("ичними", 80, 1, None),
    Among("уальними", 80, 1, None),
    Among("ійними", 80, 1, None),
    Among("аційними", 84, 1, None),
    Among("онними", 80, 1, None),
    Among("ій", -1, 2, None),
    Among("ічній", 87, 1, None),
    Among("ичній", 87, 1, None),
    Among("уальній", 87, 1, None),
    Among("ійній", 87, 1, None),
    Among("аційній", 91, 1, None),
    Among("онній", 87, 1, None),
    Among("ий", -1, 2, None),
    Among("учий", 94, 2, None),
    Among("ючий", 94, 2, None),
    Among("вший", 94, 2, None),
    Among("ічний", 94, 1, None),
    Among("ичний", 94, 1, None),
    Among("уальний", 94, 1, None),
    Among("уваний", 94, 2, None),
    Among("ований", 94, 2, None),
    Among("ійний", 94, 1, None),
    Among("аційний", 103, 1, None),
    Among("онний", 94, 1, None),
    Among("ой", -1, 2, None),
    Among("аціям", -1, 1, None),
    Among("ім", -1, 2, None),
    Among("їм", -1, 2, None),
    Among("им", -1, 2, None),
    Among("ічним", 110, 1, None),
    Among("ичним", 110, 1, None),
    Among("уальним", 110, 1, None),
    Among("ійним", 110, 1, None),
    Among("аційним", 114, 1, None),
    Among("онним", 110, 1, None),
    Among("єго", -1, 2, None),
    Among("ого", -1, 2, None),
    Among("ьього", 118, 2, None),
    Among("ічного", 118, 1, None),
    Among("ичного", 118, 1, None),
    Among("уального", 118, 1, None),
    Among("ійного", 118, 1, None),
    Among("аційного", 123, 1, None),
    Among("онного", 118, 1, None),
    Among("увано", -1, 2, None),
    Among("овано", -1, 2, None),
];

static A_3: &'static [Among<Context>; 34] = &[
    Among("у", -1, 1, None),
    Among("єш", -1, 1, None),
    Among("еш", -1, 1, None),
    Among("иш", -1, 1, None),
    Among("уть", -1, 1, None),
    Among("ють", -1, 1, None),
    Among("ають", 5, 1, None),
    Among("ю", -1, 1, None),
    Among("аю", 7, 1, None),
    Among("є", -1, 1, None),
    Among("ла", -1, 1, None),
    Among("в", -1, 1, None),
    Among("ів", 11, 1, None),
    Among("ав", 11, 1, None),
    Among("ив", 11, 1, None),
    Among("єте", -1, 1, None),
    Among("іте", -1, 1, None),
    Among("ете", -1, 1, None),
    Among("ите", -1, 1, None),
    Among("йте", -1, 1, None),
    Among("ти", -1, 1, None),
    Among("іти", 20, 1, None),
    Among("ати", 20, 1, None),
    Among("ити", 20, 1, None),
    Among("ли", -1, 1, None),
    Among("й", -1, 1, None),
    Among("єм", -1, 1, None),
    Among("ім", -1, 1, None),
    Among("им", -1, 1, None),
    Among("ло", -1, 1, None),
    Among("ємо", -1, 1, None),
    Among("імо", -1, 1, None),
    Among("емо", -1, 1, None),
    Among("имо", -1, 1, None),
];

static A_4: &'static [Among<Context>; 30] = &[
    Among("у", -1, 2, None),
    Among("ях", -1, 2, None),
    Among("ах", -1, 2, None),
    Among("ець", -1, 1, None),
    Among("ю", -1, 2, None),
    Among("єю", 4, 2, None),
    Among("ею", 4, 2, None),
    Among("ою", 4, 2, None),
    Among("я", -1, 2, None),
    Among("є", -1, 2, None),
    Among("і", -1, 2, None),
    Among("еві", 10, 2, None),
    Among("ові", 10, 2, None),
    Among("ї", -1, 2, None),
    Among("а", -1, 2, None),
    Among("ів", -1, 2, None),
    Among("ев", -1, 2, None),
    Among("е", -1, 2, None),
    Among("и", -1, 2, None),
    Among("ями", 18, 2, None),
    Among("ами", 18, 2, None),
    Among("ей", -1, 2, None),
    Among("ям", -1, 2, None),
    Among("єм", -1, 2, None),
    Among("ам", -1, 2, None),
    Among("ем", -1, 2, None),
    Among("ом", -1, 2, None),
    Among("о", -1, 2, None),
    Among("тво", 27, 2, None),
    Among("ство", 28, 2, None),
];

static A_5: &'static [Among<Context>; 7] = &[
    Among("яр", -1, 1, None),
    Among("ар", -1, 1, None),
    Among("ач", -1, 1, None),
    Among("ник", -1, 1, None),
    Among("альник", 3, 1, None),
    Among("ельник", 3, 1, None),
    Among("івник", 3, 1, None),
];

static A_6: &'static [Among<Context>; 61] = &[
    Among("еньку", -1, 1, None),
    Among("усь", -1, 1, None),
    Among("ісь", -1, 1, None),
    Among("ось", -1, 1, None),
    Among("унь", -1, 1, None),
    Among("інь", -1, 1, None),
    Among("ень", -1, 1, None),
    Among("еня", -1, 1, None),
    Among("ічкі", -1, 1, None),
    Among("ечкі", -1, 1, None),
    Among("очечкі", 9, 1, None),
    Among("очкі", -1, 1, None),
    Among("інькі", -1, 1, None),
    Among("юсінькі", 12, 1, None),
    Among("ісінькі", 12, 1, None),
    Among("енькі", -1, 1, None),
    Among("есенькі", 15, 1, None),
    Among("онькі", -1, 1, None),
    Among("ічка", -1, 1, None),
    Among("ечка", -1, 1, None),
    Among("очечка", 19, 1, None),
    Among("очка", -1, 1, None),
    Among("інька", -1, 1, None),
    Among("юсінька", 22, 1, None),
    Among("ісінька", 22, 1, None),
    Among("енька", -1, 1, None),
    Among("есенька", 25, 1, None),
    Among("онька", -1, 1, None),
    Among("інка", -1, 1, None),
    Among("инка", -1, 1, None),
    Among("ічки", -1, 1, None),
    Among("ечки", -1, 1, None),
    Among("очечки", 31, 1, None),
    Among("очки", -1, 1, None),
    Among("іньки", -1, 1, None),
    Among("юсіньки", 34, 1, None),
    Among("ісіньки", 34, 1, None),
    Among("еньки", -1, 1, None),
    Among("есеньки", 37, 1, None),
    Among("оньки", -1, 1, None),
    Among("ік", -1, 1, None),
    Among("ик", -1, 1, None),
    Among("чик", 41, 1, None),
    Among("ок", -1, 1, None),
    Among("ічок", 43, 1, None),
    Among("ечок", 43, 1, None),
    Among("очечок", 45, 1, None),
    Among("очок", 43, 1, None),
    Among("ятко", -1, 1, None),
    Among("ітко", -1, 1, None),
    Among("атко", -1, 1, None),
    Among("ічко", -1, 1, None),
    Among("ечко", -1, 1, None),
    Among("очечко", 52, 1, None),
    Among("очко", -1, 1, None),
    Among("інько", -1, 1, None),
    Among("юсінько", 55, 1, None),
    Among("ісінько", 55, 1, None),
    Among("енько", -1, 1, None),
    Among("есенько", 58, 1, None),
    Among("онько", -1, 1, None),
];

static A_7: &'static [Among<Context>; 9] = &[
    Among("очеч", -1, 1, None),
    Among("ітк", -1, 1, None),
    Among("атк", -1, 1, None),
    Among("іньк", -1, 1, None),
    Among("юсіньк", 3, 1, None),
    Among("ісіньк", 3, 1, None),
    Among("еньк", -1, 1, None),
    Among("есеньк", 6, 1, None),
    Among("оньк", -1, 1, None),
];

static A_8: &'static [Among<Context>; 4] = &[
    Among("яння", -1, 1, None),
    Among("іння", -1, 1, None),
    Among("ання", -1, 1, None),
    Among("ення", -1, 1, None),
];

static A_9: &'static [Among<Context>; 23] = &[
    Among("іст", -1, 1, None),
    Among("уват", -1, 1, None),
    Among("оват", -1, 1, None),
    Among("ищ", -1, 1, None),
    Among("ість", -1, 1, None),
    Among("овість", 4, 1, None),
    Among("ність", 4, 1, None),
    Among("ниця", -1, 1, None),
    Among("ація", -1, 1, None),
    Among("ізація", 8, 1, None),
    Among("арня", -1, 1, None),
    Among("івня", -1, 1, None),
    Among("івщина", -1, 1, None),
    Among("ов", -1, 1, None),
    Among("ськ", -1, 1, None),
    Among("зьк", -1, 1, None),
    Among("ізм", -1, 1, None),
    Among("льн", -1, 1, None),
    Among("ельн", 17, 1, None),
    Among("ян", -1, 1, None),
    Among("івн", -1, 1, None),
    Among("ство", -1, 1, None),
    Among("цтво", -1, 1, None),
];

static A_10: &'static [Among<Context>; 4] = &[
    Among("іш", -1, 1, None),
    Among("ь", -1, 4, None),
    Among("л", -1, 2, None),
    Among("н", -1, 3, None),
];

static G_v: &'static [u8; 5] = &[33, 65, 8, 192, 208];

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut i_minStem : i32;
    context.i_pV = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        if !env.go_out_grouping(G_v, 1072, 1111) {
            break 'lab0;
        }
        env.next_char();
        context.i_pV = env.cursor;
        if !env.go_in_grouping(G_v, 1072, 1111) {
            break 'lab0;
        }
        env.next_char();
        if !env.go_out_grouping(G_v, 1072, 1111) {
            break 'lab0;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 1072, 1111) {
            break 'lab0;
        }
        env.next_char();
        context.i_p2 = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    i_minStem = 0;
    let v_2 = env.cursor;
    'lab1: loop {
        if env.cursor > i_minStem {
            break 'lab1;
        }
        env.cursor = i_minStem;
        if !env.hop(3) {
            break 'lab1;
        }
        i_minStem = env.cursor;
        break 'lab1;
    }
    env.cursor = v_2;
    'lab2: loop {
        if context.i_pV >= i_minStem{
            break 'lab2;
        }
        context.i_pV = i_minStem;
        break 'lab2;
    }
    return true
}

fn r_R2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p2 <= env.cursor
}

fn r_perfective_gerund(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 5 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 140 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 184 as u8)) {
        return false;
    }

    among_var = env.find_among_b(A_0, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            'lab0: loop {
                let v_1 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"а") {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                if !env.eq_s_b(&"я") {
                    return false;
                }
                break 'lab0;
            }
            env.slice_del();
        }
        2 => {
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_reflexive(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 3 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 140 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 143 as u8)) {
        return false;
    }

    if env.find_among_b(A_1, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true
}

fn r_adjective(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
        }
        2 => {
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_verb(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_3, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true
}

fn r_noun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_4, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_from("ц");
        }
        2 => {
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_professional(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_5, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true
}

fn r_diminutive(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_6, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true
}

fn r_diminutive_stem(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 5 <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 135 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 186 as u8)) {
        return false;
    }

    if env.find_among_b(A_7, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true
}

fn r_verbal_noun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 7 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 143 as u8) {
        return false;
    }

    if env.find_among_b(A_8, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R2(env, context) {
        return false;
    }
    env.slice_del();
    return true
}

fn r_derivational(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_9, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R2(env, context) {
        return false;
    }
    env.slice_del();
    return true
}

fn r_tidy_up(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_10, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_del();
            env.ket = env.cursor;
            if !env.eq_s_b(&"н") {
                return false;
            }
            env.bra = env.cursor;
            if !env.eq_s_b(&"н") {
                return false;
            }
            env.slice_del();
        }
        2 => {
            'lab0: loop {
                let v_1 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"б") {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                'lab2: loop {
                    if !env.eq_s_b(&"п") {
                        break 'lab2;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                'lab3: loop {
                    if !env.eq_s_b(&"в") {
                        break 'lab3;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                if !env.eq_s_b(&"м") {
                    return false;
                }
                break 'lab0;
            }
            env.slice_del();
        }
        3 => {
            if !env.eq_s_b(&"н") {
                return false;
            }
            env.slice_del();
        }
        4 => {
            env.slice_del();
        }
        _ => ()
    }
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p2: 0,
        i_pV: 0,
    };
    let mut b_found_professional : bool;
    let mut b_found_diminutive : bool;
    let v_1 = env.cursor;
    'lab0: loop {
        env.bra = env.cursor;
        if !env.eq_s(&"най") {
            break 'lab0;
        }
        env.ket = env.cursor;
        env.slice_del();
        break 'lab0;
    }
    env.cursor = v_1;
    'lab1: loop {
        let v_2 = env.cursor;
        'lab2: loop {
            let v_3 = env.cursor;
            if !env.eq_s(&"окремо") {
                break 'lab2;
            }
            if env.cursor < env.limit {
                break 'lab2;
            }
            env.cursor = v_3;
            env.limit_backward = env.cursor;
            env.cursor = env.limit;
            env.ket = env.cursor;
            if !env.eq_s_b(&"о") {
                break 'lab2;
            }
            env.bra = env.cursor;
            env.slice_del();
            env.cursor = env.limit_backward;
            break 'lab1;
        }
        env.cursor = v_2;
        b_found_diminutive = false;
        b_found_professional = false;
        r_mark_regions(env, context);
        env.limit_backward = env.cursor;
        env.cursor = env.limit;
        if env.cursor < context.i_pV {
            return false;
        }
        let v_4 = env.limit_backward;
        env.limit_backward = context.i_pV;
        let v_5 = env.limit - env.cursor;
        'lab3: loop {
            'lab4: loop {
                let v_6 = env.limit - env.cursor;
                'lab5: loop {
                    if !r_perfective_gerund(env, context) {
                        break 'lab5;
                    }
                    break 'lab4;
                }
                env.cursor = env.limit - v_6;
                let v_7 = env.limit - env.cursor;
                'lab6: loop {
                    if !r_reflexive(env, context) {
                        env.cursor = env.limit - v_7;
                        break 'lab6;
                    }
                    break 'lab6;
                }
                'lab7: loop {
                    let v_8 = env.limit - env.cursor;
                    'lab8: loop {
                        if !r_verbal_noun(env, context) {
                            break 'lab8;
                        }
                        break 'lab7;
                    }
                    env.cursor = env.limit - v_8;
                    let v_9 = env.limit - env.cursor;
                    'lab9: loop {
                        'lab10: loop {
                            let v_10 = env.limit - env.cursor;
                            'lab11: loop {
                                if !r_professional(env, context) {
                                    break 'lab11;
                                }
                                b_found_professional = true;
                                break 'lab10;
                            }
                            env.cursor = env.limit - v_10;
                            if !r_diminutive(env, context) {
                                env.cursor = env.limit - v_9;
                                break 'lab9;
                            }
                            b_found_diminutive = true;
                            break 'lab10;
                        }
                        break 'lab9;
                    }
                    if b_found_professional {
                        break 'lab3;
                    }
                    'lab12: loop {
                        let v_11 = env.limit - env.cursor;
                        'lab13: loop {
                            if !r_adjective(env, context) {
                                break 'lab13;
                            }
                            break 'lab12;
                        }
                        env.cursor = env.limit - v_11;
                        'lab14: loop {
                            if !r_verb(env, context) {
                                break 'lab14;
                            }
                            break 'lab12;
                        }
                        env.cursor = env.limit - v_11;
                        if !r_noun(env, context) {
                            break 'lab3;
                        }
                        break 'lab12;
                    }
                    break 'lab7;
                }
                break 'lab4;
            }
            break 'lab3;
        }
        env.cursor = env.limit - v_5;
        let v_12 = env.limit - env.cursor;
        'lab15: loop {
            if !r_diminutive_stem(env, context) {
                env.cursor = env.limit - v_12;
                break 'lab15;
            }
            b_found_diminutive = true;
            break 'lab15;
        }
        if b_found_diminutive {
            env.limit_backward = v_4;
            return false;
        }
        if b_found_professional {
            env.limit_backward = v_4;
            return false;
        }
        let v_13 = env.limit - env.cursor;
        'lab16: loop {
            if !r_professional(env, context) {
                env.cursor = env.limit - v_13;
                break 'lab16;
            }
            break 'lab16;
        }
        let v_14 = env.limit - env.cursor;
        r_derivational(env, context);
        env.cursor = env.limit - v_14;
        let v_15 = env.limit - env.cursor;
        r_tidy_up(env, context);
        env.cursor = env.limit - v_15;
        env.limit_backward = v_4;
        env.cursor = env.limit_backward;
        break 'lab1;
    }
    return true
}
