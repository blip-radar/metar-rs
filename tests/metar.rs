use metar::Metar;

#[test]
fn test_display() {
    let metar_str = "EDDM 222020Z AUTO VRB01KT CAVOK 20/13 Q1017 NOSIG";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str =
        "EDDM 231420Z AUTO 27008KT 9999 -TSRA SCT///CB 24/18 Q1013 TEMPO 28020G35KT 3500 TSRA";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "EDDM 231520Z AUTO 25012KT CAVOK 24/19 Q1012 RETSRA";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str =
        "EDDM 231550Z AUTO 27010KT 240V300 9999 TSRA BKN///CB 24/19 Q1013 TEMPO 28015G25KT";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "EKVG 232250Z AUTO 31006KT 1000 R12/0800N R30/P1500D BR OVC001/// 09/09 Q0995 RMK OVC000/// WIND SKEID 29012KT";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "BGGH 232250Z 21007KT 0700 R22/P2000N -RA FG FEW002 BKN004 OVC006 05/05 Q1005";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "LFVP 232230Z AUTO 24009KT 0450 R26/0800N FG VV/// 11/11 Q1015";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "LESA 232230Z AUTO 27010KT 230V300 8000 -TSRA //////CB 20/16 Q1023";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "EDSB 242150Z AUTO 18003KT 9999 NCD 20/14 Q1015";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "EDMA 242150Z AUTO 00000KT 9999 // FEW130/// 16/13 Q1016";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "EDNY 242150Z AUTO VRB01KT 9999 // NCD 20/15 Q1017";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "ETSN 242120Z 30004KT 9999 FEW330 19/12 Q1016 BLU+";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "ETSI 242120Z AUTO 22001KT //// // ////// 19/13 Q1015 ///";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "EDBC 032220Z AUTO 24025KT 9999 VCSH BKN034 OVC039 FEW///CB 00/M04 Q0999";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "ESSP 032220Z AUTO 02012KT 1200 R09/P1500N R27/P1500N -SN FEW003/// BKN006/// OVC010/// M02/M03 Q0990 RESHUP RESN";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "EDLW 032220Z AUTO 23012KT 3900 // SCT006/// BKN009/// OVC018/// M00/M01 Q1005";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "ETSB 032220Z AUTO /////KT //// // ////// ///// Q//// ///";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());

    let metar_str = "ETSN 261720Z 32003KT 9999 -RA FEW020 SCT070 BKN090 17/15 Q1014 RERA BLU";
    let metar = Metar::parse(metar_str).unwrap();
    assert_eq!(metar_str, metar.to_string());
}
