pub struct Netlist {
    pub name: String,
    pub includes: Vec<String>,
}

pub struct Subckt {
    pub name: String,
    pub pins: Vec<String>,
    pub params: Vec<(String, String)>,
    pub devices: Vec<Device>,
}

pub struct Device {
    pub name: String,
    pub model: String,
    pub terminals: Vec<String>,
    pub params: Vec<Param>,
}

impl Device {
    pub fn from_cp(
        name: String,
        model: String,
        terminals: Vec<String>,
        value: i32,
        unit: Unit,
    ) -> Self {
        let mut params = vec![Param {
            name: "c".to_string(),
            value: value.to_string(),
            unit: Some(unit),
        }];
        Self {
            name,
            model,
            terminals,
            params,
        }
    }
}

pub struct Param {
    pub name: String,
    pub value: String,
    pub unit: Option<Unit>,
}

pub enum Unit {
    n,
    u,
    m,
    k,
    M,
    G,
    T,
}

impl Param {
    pub fn new(name: String, value: String, unit: Option<Unit>) -> Self {
        Self { name, value, unit }
    }
}

mod tests {
    #[test]
    fn test_include() {
        use crate::ebnf::aucdl;

        let text = r#"
        ****
        .INC /EDA/PDK/stdcell/TSMC65LP/TSMCHOME/digital/Back_End/spice/tcbn65lp_200a/tcbn65lp_200a.spi
        .INC /EDA/PDK/TSMC65nm/TSMC65LP_1P9M_6X1Z1U/Calibre/lvs/source.added
        "#;
        let s = aucdl::IncludesParser::new().parse(text).unwrap();
        println!("{:?}", s);
    }

    #[test]
    fn test_subckt() {
        let text = r#".SUBCKT PLL_CP_OP2 IB40U PDA PDB SUB_N SUB_P VDD VINN VINP VOUT VSS
        *.PININFO IB40U:I PDA:I PDB:I SUB_N:I SUB_P:I VDD:I VINN:I VINP:I VSS:I VOUT:O
        XR0 net13 dumy11 rppolywo l=7u w=1u m=1
        MM30 dumy8 dumy8 dumy8 SUB_P pch_lvt l=500n w=6u m=8
        MM23 dumy12 VINP dumy8 SUB_P pch_lvt l=500n w=6u m=4
        MM24 dumy13 VINN dumy8 SUB_P pch_lvt l=500n w=6u m=4
        MM49 dumy3 dumy4 VDD SUB_P pch l=500n w=4u m=3
        MM48 dumy7 dumy4 VDD SUB_P pch l=500n w=4u m=4
        MM46 VBN dumy4 VDD SUB_P pch l=500n w=4u m=2
        MM45 net13 PDA IB40U SUB_P pch l=60n w=1u m=10
        MM44 dumy8 dumy4 VDD SUB_P pch l=500n w=4u m=2
        MM43 VOUT VBP dumy2 SUB_P pch l=200n w=4u m=3
        MM42 dumy5 VBP dumy3 SUB_P pch l=200n w=4u m=3
        MM41 dumy2 dumy4 VDD SUB_P pch l=500n w=4u m=3
        MM40 VDD VDD dumy1 SUB_P pch l=200n w=4u m=1
        MM39 VDD VDD VDD SUB_P pch l=200n w=4u m=5
        MM38 VBP VBP dumy1 SUB_P pch l=200n w=4u m=3
        MM37 dumy1 VBP VDD SUB_P pch l=1u w=4u m=1
        MM36 dumy4 VDD VDD SUB_P pch l=500n w=4u m=1
        MM35 dumy3 VDD VDD SUB_P pch l=500n w=4u m=1
        MM28 dumy4 PDB VDD SUB_P pch l=500n w=4u m=1
        MM27 dumy2 VDD VDD SUB_P pch l=500n w=4u m=1
        MM26 dumy1 VDD VDD SUB_P pch l=500n w=4u m=1
        MM25 dumy4 VBP dumy7 SUB_P pch l=200n w=4u m=6
        MM29 VDD VDD dumy3 SUB_P pch l=200n w=4u m=1
        MM31 VDD VDD dumy2 SUB_P pch l=200n w=4u m=1
        MM32 dumy5 VDD VDD SUB_P pch l=200n w=4u m=1
        MM33 VOUT VDD VDD SUB_P pch l=200n w=4u m=1
        MM34 dumy7 VDD VDD SUB_P pch l=200n w=4u m=2
        MM47 VBP VDD VDD SUB_P pch l=200n w=4u m=1
        MM74 net117 dumy11 VSS SUB_N nch_dnw l=500n w=4u m=2
        MM73 dumy9 dumy11 VSS SUB_N nch_dnw l=500n w=4u m=1
        MM72 net116 dumy11 VSS SUB_N nch_dnw l=500n w=4u m=2
        MM71 dumy11 PDA VSS SUB_N nch_dnw l=500n w=4u m=1
        MM70 dumy10 dumy11 VSS SUB_N nch_dnw l=500n w=4u m=1
        MM69 dumy12 dumy5 VSS SUB_N nch_dnw l=500n w=4u m=2
        MM68 dumy13 dumy5 VSS SUB_N nch_dnw l=500n w=4u m=2
        MM67 VOUT VBN dumy13 SUB_N nch_dnw l=200n w=4u m=2
        MM66 dumy5 VBN dumy12 SUB_N nch_dnw l=200n w=4u m=2
        MM65 net118 VBN VSS SUB_N nch_dnw l=3u w=3u m=1
        MM64 VBN VBN net118 SUB_N nch_dnw l=200n w=4u m=2
        MM63 dumy4 net13 net117 SUB_N nch_dnw l=200n w=4u m=2
        MM62 VBP net13 dumy10 SUB_N nch_dnw l=200n w=4u m=1
        MM61 dumy11 net13 net116 SUB_N nch_dnw l=200n w=4u m=2
        MM60 VSS VSS VSS SUB_N nch_dnw l=200n w=4u m=12
        MM59 VBP VSS VSS SUB_N nch_dnw l=200n w=4u m=1
        MM58 dumy10 VSS VSS SUB_N nch_dnw l=200n w=4u m=1
        MM57 dumy11 VSS VSS SUB_N nch_dnw l=200n w=4u m=2
        MM56 dumy4 VSS VSS SUB_N nch_dnw l=200n w=4u m=2
        MM55 VSS VSS dumy12 SUB_N nch_dnw l=200n w=4u m=2
        MM54 VSS VSS dumy13 SUB_N nch_dnw l=200n w=4u m=2
        MM53 VSS VSS VBN SUB_N nch_dnw l=200n w=4u m=2
        MM52 VSS VSS dumy10 SUB_N nch_dnw l=500n w=4u m=1
        MM51 VSS VSS dumy9 SUB_N nch_dnw l=500n w=4u m=1
        MM50 VSS VSS VSS SUB_N nch_dnw l=500n w=4u m=2
        MM77 dumy2 VINN dumy9 SUB_N nch_lvt_dnw l=500n w=3u m=4
        MM76 dumy3 VINP dumy9 SUB_N nch_lvt_dnw l=500n w=3u m=4
        MM75 dumy9 dumy9 dumy9 SUB_N nch_lvt_dnw l=500n w=3u m=8
        .ENDS"#;
        let s = aucdl::SubcktParser::new().parse(text).unwrap();
        println!("{:?}", s);
    }
}
