use std::time::Instant;

use ::argh::FromArgs;
use ::clap::Parser;

mod argh;
mod clap;

fn main() {
    let v = [
        "--", "--arg1", "<ARG1>", "--arg2", "<ARG2>", "--arg3", "<ARG3>", "--arg4", "<ARG4>",
        "--arg5", "<ARG5>", "--arg6", "<ARG6>", "--arg7", "<ARG7>", "--arg8", "<ARG8>", "--arg9",
        "<ARG9>", "--arg10", "<ARG10>", "--arg11", "<ARG11>", "--arg12", "<ARG12>", "--arg13",
        "<ARG13>", "--arg14", "<ARG14>", "--arg15", "<ARG15>", "--arg16", "<ARG16>", "--arg17",
        "<ARG17>", "--arg18", "<ARG18>", "--arg19", "<ARG19>", "--arg20", "<ARG20>", "--arg21",
        "<ARG21>", "--arg22", "<ARG22>", "--arg23", "<ARG23>", "--arg24", "<ARG24>", "--arg25",
        "<ARG25>", "--arg26", "<ARG26>", "--arg27", "<ARG27>", "--arg28", "<ARG28>", "--arg29",
        "<ARG29>", "--arg30", "<ARG30>", "--arg40", "<ARG40>", "--arg41", "<ARG41>", "--arg42",
        "<ARG42>", "--arg43", "<ARG43>", "--arg44", "<ARG44>", "--arg45", "<ARG45>", "--arg46",
        "<ARG46>", "--arg47", "<ARG47>", "--arg48", "<ARG48>", "--arg49", "<ARG49>", "--arg50",
        "<ARG50>", "--arg51", "<ARG51>", "--arg52", "<ARG52>", "--arg53", "<ARG53>", "--arg54",
        "<ARG54>", "--arg55", "<ARG55>", "--arg56", "<ARG56>", "--arg57", "<ARG57>", "--arg58",
        "<ARG58>", "--arg59", "<ARG59>", "--arg60", "<ARG60>", "--arg61", "<ARG61>", "--arg62",
        "<ARG62>", "--arg63", "<ARG63>", "--arg64", "<ARG64>", "--arg65", "<ARG65>", "--arg66",
        "<ARG66>", "--arg67", "<ARG67>", "--arg68", "<ARG68>", "--arg69", "<ARG69>", "--arg70",
        "<ARG70>", "--arg71", "<ARG71>", "--arg72", "<ARG72>", "--arg73", "<ARG73>", "--arg74",
        "<ARG74>", "--arg75", "<ARG75>", "--arg76", "<ARG76>", "--arg77", "<ARG77>", "--arg78",
        "<ARG78>", "--arg79", "<ARG79>", "--arg80", "<ARG80>", "--arg81", "<ARG81>", "--arg82",
        "<ARG82>", "--arg83", "<ARG83>", "--arg84", "<ARG84>", "--arg85", "<ARG85>", "--arg86",
        "<ARG86>", "--arg87", "<ARG87>", "--arg88", "<ARG88>", "--arg89", "<ARG89>", "--arg90",
        "<ARG90>", "--arg91", "<ARG91>", "--arg92", "<ARG92>", "--arg93", "<ARG93>", "--arg94",
        "<ARG94>", "--arg95", "<ARG95>", "--arg96", "<ARG96>", "--arg97", "<ARG97>", "--arg98",
        "<ARG98>", "--arg99", "<ARG99>", "--arg100", "<ARG100>", "--arg101", "<ARG101>",
        "--arg102", "<ARG102>", "--arg103", "<ARG103>", "--arg104", "<ARG104>", "--arg105",
        "<ARG105>", "--arg106", "<ARG106>", "--arg107", "<ARG107>", "--arg108", "<ARG108>",
        "--arg109", "<ARG109>", "--arg110", "<ARG110>", "--arg111", "<ARG111>", "--arg112",
        "<ARG112>", "--arg113", "<ARG113>", "--arg114", "<ARG114>", "--arg115", "<ARG115>",
        "--arg116", "<ARG116>", "--arg117", "<ARG117>", "--arg118", "<ARG118>", "--arg119",
        "<ARG119>", "--arg120", "<ARG120>", "--arg121", "<ARG121>", "--arg122", "<ARG122>",
        "--arg123", "<ARG123>", "--arg124", "<ARG124>", "--arg125", "<ARG125>", "--arg126",
        "<ARG126>", "--arg127", "<ARG127>", "--arg128", "<ARG128>", "--arg129", "<ARG129>",
        "--arg130", "<ARG130>", "--arg131", "<ARG131>", "--arg132", "<ARG132>", "--arg133",
        "<ARG133>", "--arg134", "<ARG134>", "--arg135", "<ARG135>", "--arg136", "<ARG136>",
        "--arg137", "<ARG137>", "--arg138", "<ARG138>", "--arg139", "<ARG139>", "--arg140",
        "<ARG140>", "--arg141", "<ARG141>", "--arg142", "<ARG142>", "--arg143", "<ARG143>",
        "--arg144", "<ARG144>", "--arg145", "<ARG145>", "--arg146", "<ARG146>", "--arg147",
        "<ARG147>", "--arg148", "<ARG148>", "--arg149", "<ARG149>", "--arg150", "<ARG150>",
        "--arg151", "<ARG151>", "--arg152", "<ARG152>", "--arg153", "<ARG153>", "--arg154",
        "<ARG154>", "--arg155", "<ARG155>", "--arg156", "<ARG156>", "--arg157", "<ARG157>",
        "--arg158", "<ARG158>", "--arg159", "<ARG159>", "--arg160", "<ARG160>", "--arg161",
        "<ARG161>", "--arg162", "<ARG162>", "--arg163", "<ARG163>", "--arg164", "<ARG164>",
        "--arg165", "<ARG165>", "--arg166", "<ARG166>", "--arg167", "<ARG167>", "--arg168",
        "<ARG168>", "--arg169", "<ARG169>", "--arg170", "<ARG170>", "--arg171", "<ARG171>",
        "--arg172", "<ARG172>", "--arg173", "<ARG173>", "--arg174", "<ARG174>", "--arg175",
        "<ARG175>", "--arg176", "<ARG176>", "--arg177", "<ARG177>", "--arg178", "<ARG178>",
        "--arg179", "<ARG179>",
    ];
    // std::env::set_var("RUST_BACKTRACE", "1");

    let start = Instant::now();
    let _clap_config = dbg!(clap::A::parse_from(v.iter()));
    let clap_duration = start.elapsed();

    let start = Instant::now();
    let _argh_config = dbg!(argh::A::from_args(&["command_name"], &v[1..]).unwrap());
    let argh_duration = start.elapsed();

    println!("clap parse duration: {clap_duration:?}");
    println!("argh parse duration: {argh_duration:?}");
}
