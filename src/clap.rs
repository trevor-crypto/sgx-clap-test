use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct A {
    #[clap(flatten)]
    args1: Args1,
    #[clap(flatten)]
    args2: Args2,
}

#[derive(Debug, Clone, Parser)]
pub struct Args1 {
    #[arg(long)]
    arg1: String,
    #[arg(long)]
    arg2: String,
    #[arg(long)]
    arg3: String,
    #[arg(long)]
    arg4: String,
    #[arg(long)]
    arg5: String,
    #[arg(long)]
    arg6: String,
    #[arg(long)]
    arg7: String,
    #[arg(long)]
    arg8: String,
    #[arg(long)]
    arg9: String,
    #[arg(long)]
    arg10: String,
    #[arg(long)]
    arg11: String,
    #[arg(long)]
    arg12: String,
    #[arg(long)]
    arg13: String,
    #[arg(long)]
    arg14: String,
    #[arg(long)]
    arg15: String,
    #[arg(long)]
    arg16: String,
    #[arg(long)]
    arg17: String,
    #[arg(long)]
    arg18: String,
    #[arg(long)]
    arg19: String,
    #[arg(long)]
    arg20: String,
    #[arg(long)]
    arg21: String,
    #[arg(long)]
    arg22: String,
    #[arg(long)]
    arg23: String,
    #[arg(long)]
    arg24: String,
    #[arg(long)]
    arg25: String,
    #[arg(long)]
    arg26: String,
    #[arg(long)]
    arg27: String,
    #[arg(long)]
    arg28: String,
    #[arg(long)]
    arg29: String,
    #[arg(long)]
    arg30: String,
    #[arg(long)]
    arg40: String,
    #[arg(long)]
    arg41: String,
    #[arg(long)]
    arg42: String,
    #[arg(long)]
    arg43: String,
    #[arg(long)]
    arg44: String,
    #[arg(long)]
    arg45: String,
    #[arg(long)]
    arg46: String,
    #[arg(long)]
    arg47: String,
    #[arg(long)]
    arg48: String,
    #[arg(long)]
    arg49: String,
    #[arg(long)]
    arg50: String,
    #[arg(long)]
    arg51: String,
    #[arg(long)]
    arg52: String,
    #[arg(long)]
    arg53: String,
    #[arg(long)]
    arg54: String,
    #[arg(long)]
    arg55: String,
    #[arg(long)]
    arg56: String,
    #[arg(long)]
    arg57: String,
    #[arg(long)]
    arg58: String,
    #[arg(long)]
    arg59: String,
    #[arg(long)]
    arg60: String,
    #[arg(long)]
    arg61: String,
    #[arg(long)]
    arg62: String,
    #[arg(long)]
    arg63: String,
    #[arg(long)]
    arg64: String,
    #[arg(long)]
    arg65: String,
    #[arg(long)]
    arg66: String,
    #[arg(long)]
    arg67: String,
    #[arg(long)]
    arg68: String,
    #[arg(long)]
    arg69: String,
    #[arg(long)]
    arg70: String,
    #[arg(long)]
    arg71: String,
    #[arg(long)]
    arg72: String,
    #[arg(long)]
    arg73: String,
    #[arg(long)]
    arg74: String,
    #[arg(long)]
    arg75: String,
    #[arg(long)]
    arg76: String,
    #[arg(long)]
    arg77: String,
    #[arg(long)]
    arg78: String,
    #[arg(long)]
    arg79: String,
    #[arg(long)]
    arg80: String,
    #[arg(long)]
    arg81: String,
    #[arg(long)]
    arg82: String,
    #[arg(long)]
    arg83: String,
    #[arg(long)]
    arg84: String,
    #[arg(long)]
    arg85: String,
    #[arg(long)]
    arg86: String,
    #[arg(long)]
    arg87: String,
    #[arg(long)]
    arg88: String,
    #[arg(long)]
    arg89: String,
}

#[derive(Debug, Clone, Parser)]
pub struct Args2 {
    #[arg(long)]
    arg90: String,
    #[arg(long)]
    arg91: String,
    #[arg(long)]
    arg92: String,
    #[arg(long)]
    arg93: String,
    #[arg(long)]
    arg94: String,
    #[arg(long)]
    arg95: String,
    #[arg(long)]
    arg96: String,
    #[arg(long)]
    arg97: String,
    #[arg(long)]
    arg98: String,
    #[arg(long)]
    arg99: String,
    #[arg(long)]
    arg100: String,
    #[arg(long)]
    arg101: String,
    #[arg(long)]
    arg102: String,
    #[arg(long)]
    arg103: String,
    #[arg(long)]
    arg104: String,
    #[arg(long)]
    arg105: String,
    #[arg(long)]
    arg106: String,
    #[arg(long)]
    arg107: String,
    #[arg(long)]
    arg108: String,
    #[arg(long)]
    arg109: String,
    #[arg(long)]
    arg110: String,
    #[arg(long)]
    arg111: String,
    #[arg(long)]
    arg112: String,
    #[arg(long)]
    arg113: String,
    #[arg(long)]
    arg114: String,
    #[arg(long)]
    arg115: String,
    #[arg(long)]
    arg116: String,
    #[arg(long)]
    arg117: String,
    #[arg(long)]
    arg118: String,
    #[arg(long)]
    arg119: String,
    #[arg(long)]
    arg120: String,
    #[arg(long)]
    arg121: String,
    #[arg(long)]
    arg122: String,
    #[arg(long)]
    arg123: String,
    #[arg(long)]
    arg124: String,
    #[arg(long)]
    arg125: String,
    #[arg(long)]
    arg126: String,
    #[arg(long)]
    arg127: String,
    #[arg(long)]
    arg128: String,
    #[arg(long)]
    arg129: String,
    #[arg(long)]
    arg130: String,
    #[arg(long)]
    arg131: String,
    #[arg(long)]
    arg132: String,
    #[arg(long)]
    arg133: String,
    #[arg(long)]
    arg134: String,
    #[arg(long)]
    arg135: String,
    #[arg(long)]
    arg136: String,
    #[arg(long)]
    arg137: String,
    #[arg(long)]
    arg138: String,
    #[arg(long)]
    arg139: String,
    #[arg(long)]
    arg140: String,
    #[arg(long)]
    arg141: String,
    #[arg(long)]
    arg142: String,
    #[arg(long)]
    arg143: String,
    #[arg(long)]
    arg144: String,
    #[arg(long)]
    arg145: String,
    #[arg(long)]
    arg146: String,
    #[arg(long)]
    arg147: String,
    #[arg(long)]
    arg148: String,
    #[arg(long)]
    arg149: String,
    #[arg(long)]
    arg150: String,
    #[arg(long)]
    arg151: String,
    #[arg(long)]
    arg152: String,
    #[arg(long)]
    arg153: String,
    #[arg(long)]
    arg154: String,
    #[arg(long)]
    arg155: String,
    #[arg(long)]
    arg156: String,
    #[arg(long)]
    arg157: String,
    #[arg(long)]
    arg158: String,
    #[arg(long)]
    arg159: String,
    #[arg(long)]
    arg160: String,
    #[arg(long)]
    arg161: String,
    #[arg(long)]
    arg162: String,
    #[arg(long)]
    arg163: String,
    #[arg(long)]
    arg164: String,
    #[arg(long)]
    arg165: String,
    #[arg(long)]
    arg166: String,
    #[arg(long)]
    arg167: String,
    #[arg(long)]
    arg168: String,
    #[arg(long)]
    arg169: String,
    #[arg(long)]
    arg170: String,
    #[arg(long)]
    arg171: String,
    #[arg(long)]
    arg172: String,
    #[arg(long)]
    arg173: String,
    #[arg(long)]
    arg174: String,
    #[arg(long)]
    arg175: String,
    #[arg(long)]
    arg176: String,
    #[arg(long)]
    arg177: String,
    #[arg(long)]
    arg178: String,
    #[arg(long)]
    arg179: String,
}
