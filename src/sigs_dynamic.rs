use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use crate::utils;

const NOHIT: &str = "NOHIT";

const APK: &str = "apk\tArchive\tAndroid\tAndroid Package";
const ART: &str = "art\tImage\tMisc\tAOL ART";
const AWD: &str = "awd\tDocument\tMisc\tMS Fax At Work Document";

const CCD_CCT: &str = "cct/ccd\tImage\tMisc\tCorelCAD Drawing/Template";

const DLL: &str = "dll\tBinary\tWindows\tMS Dynamic Link Library";
const DOC_DOT: &str = "doc/dot\tDocument\tMisc\tMS Word";
const DOCM: &str = "docm\tDocument\tMisc\tMS Word with Macros";
const DOCX: &str = "docx\tDocument\tMisc\tMS Word Document";
const DOTM: &str = "dotm\tDocument\tMisc\tMS Word Template with Macros";
const DOTX: &str = "dotx\tDocument\tMisc\tMS Word Template";
const DTP: &str = "dtp\tDocument\tMisc\tPublish-It";

const EPUB: &str = "epub\tDocument\tMisc\tElectronic Publication Document";
const EXE: &str = "exe\tExecutable\tWindows\tMS Executable";
const EXE_LE_LX: &str = "exe\tExecutable\tWindows\tMS Linear Executable";
const EXE_MSDOS: &str = "exe\tExecutable\tMS-DOS\tMS Executable";
const EXE_NE: &str = "exe\tExecutable\tMS-DOS\tMS New Executable";

const FLA_SPA: &str = "fla/spa\tDocument\tMisc\tFutureSplash Animator/Flash";
const FPX: &str = "fpx\tImage\tMisc\tKodak FlashPix";
const FTM_FTW: &str = "ftm/ftw\tDocument\tMisc\tFamily Tree Maker";

const IPT: &str = "ipt\tImage\tMisc\tAutoDesk Inventor";

const JAR: &str = "jar\tArchive\tMisc\tJava Archive";

const MAX: &str = "max\tDocument\tMisc\t3D Studio Max";
const MIX: &str = "mix\tImage\tMisc\tMS PhotoDraw";
const MPP: &str = "mpp\tDocument\tMisc\tMS Project";
const MS_COMPOUND_ZERO: &str = " \tBinary\tMisc\tMS Compound File";
const MS_THUMBSDB: &str = "db\tData\tWindows\tMS Thumbs DB";
const MSG: &str = "msg\tEmail\tWindows\tMS Outlook Email Message";
const MSI: &str = "msi\tExecutable\tWindows\tMS Installer";
const MSP: &str = "msp\tData\tWindows\tMS Windows Installer Patch";

const NUPKG: &str = "nupkg\tArchive\tWindows\tNuGet Package";

const OBD: &str = "obd\tDocument\tMisc\tMS Office Binder";
const OD: &str = "od\tDocument\tMisc\tOpenDocument Document";
const ODB: &str = "odb\tDatabase\tMisc\tOpenDocument Database";
const ODC: &str = "odc\tDocument\tMisc\tOpenDocument Chart";
const ODF: &str = "odf\tDocument\tMisc\tOpenDocument Formula";
const ODG: &str = "odg\tDocument\tMisc\tOpenDocument Graphics";
const ODI: &str = "odi\tImage\tMisc\tOpenDocument Image";
const ODM: &str = "odm\tDocument\tMisc\tOpenDocument Master Document";
const ODP: &str = "odp\tDocument\tMisc\tOpenDocument Presentation";
const ODS: &str = "ods\tDocument\tMisc\tOpenDocument Spreadsheet";
const ODT: &str = "odt\tDocument\tMisc\tOpenDocument Text";
const OFM: &str = "ofm\tDocument\tMisc\tMS Office Form";
const ORA: &str = "ora\tImage\tMisc\tOpenRaster Image";
const OTC: &str = "otc\tDocument\tMisc\tOpenDocument Chart Template";
const OTF: &str = "otf\tDocument\tMisc\tOpenDocument Formula Template";
const OTG: &str = "otg\tDocument\tMisc\tOpenDocument Graphics Template";
const OTH: &str = "oth\tDocument\tMisc\tOpenDocument HTML Template";
const OTI: &str = "oti\tDocument\tMisc\tOpenDocument Image Template";
const OTP: &str = "otp\tDocument\tMisc\tOpenDocument Presentation Template";
const OTS: &str = "ots\tDocument\tMisc\tOpenDocument Spreadsheet Template";
const OTT: &str = "ott\tDocument\tMisc\tOpenDocument Text Template";

const POTM: &str = "potm\tDocument\tMisc\tMS PowerPoint Template with Macros";
const POTX: &str = "potx\tDocument\tMisc\tMS PowerPoint Template";
const PPA_PWZ: &str = "ppa/pwz\tDocument\tMisc\tMS PowerPoint";
const PPAM: &str = "ppam\tDocument\tMisc\tMS PowerPoint Add-In with Macros";
const PPSM: &str = "ppsm\tDocument\tMisc\tMS PowerPoint Slide Show with Macros";
const PPSX: &str = "ppsx\tDocument\tMisc\tMS PowerPoint Slide Show";
const PPTX: &str = "pptx\tDocument\tMisc\tMS PowerPoint";
const PPT_PPS: &str = "ppt/pps\tDocument\tMisc\tMS PowerPoint";
const PPTM: &str = "pptm\tDocument\tMisc\tMS PowerPoint with Macros";
const PTM: &str = "ptm\tDocument\tMisc\tMS MapPoint";
const PUB: &str = "pub\tDocument\tMisc\tMS Publisher";

const SCN: &str = "scn\tImage\tMisc\tSoftimage";
const SDA: &str = "sdd\tDocument\tMisc\tStarOffice Drawing";
const SDC: &str = "sdc\tDocument\tMisc\tStarOffice Calc";
const SDD: &str = "sda\tDocument\tMisc\tStarOffice Impress";
const SDS: &str = "sds\tDocument\tMisc\tStarOffice Chart";
const SDW: &str = "sdw\tDocument\tMisc\tStarOffice Writer";
const SGL: &str = "sgl\tDocument\tMisc\tStarOffice Global Document";
const SMF: &str = "smf\tDocument\tMisc\tStarOffice Math";
const SHW: &str = "shw\tDocument\tMisc\tWordPerfect Presentation";
const SXC_STC: &str = "sxc/stc\tDocument\tMisc\tOpenOffice Calc";
const SXD: &str = "sxd\tDocument\tMisc\tOpenOffice Draw";
const SXI_STI: &str = "sxi/sti\tDocument\tMisc\tOpenOffice Impress";
const SXM: &str = "sxm\tDocument\tMisc\tOpenOffice Math";
const SXW_STW: &str = "sxw/stw\tDocument\tMisc\tOpenOffice Writer";
const SYS: &str = "sys\tBinary\tWindows\tMS System/Driver File";

const VSD: &str = "vsd\tDocument\tMisc\tMS Visio";

const WDB: &str = "wdb\tDatabase\tMisc\tMS Works Database";
const WPD: &str = "wpd\tDocument\tMisc\tWordPerfect Document";
const WPF_WORKS: &str = "wpf\tDocument\tMisc\tMS Works Word Processor";
const WPG: &str = "wpg\tImage\tMisc\tWordPerfect Graphic";
const WPS_WPT: &str = "wps/wpt\tDocument\tMisc\tMS Works";

const XLAM: &str = "xlam\tDocument\tMisc\tMS Excel Add-In with Macros";
const XLTM: &str = "xltm\tDocument\tMisc\tMS Excel Template with Macros";
const XLTX: &str = "xltx\tDocument\tMisc\tMS Excel Template";
const XLS: &str = "xls\tDocument\tMisc\tMS Excel";
const XLSX: &str = "xlsx\tDocument\tMisc\tMS Excel Document";
const XLSM: &str = "xlsm\tDocument\tMisc\tMS Excel with Macros";
const XLSB: &str = "xlsb\tDocument\tMisc\tMS Excel Binary Workbook";
const XPI: &str =
    "xpi\tArchive\tMisc\tCross-Platform Installer Module (Firefox, Thunderbird Extension)";
const XPS_OXPS: &str = "xps/oxps\tDocument\tMisc\tOpen XML Paper Specification";

pub fn check(reader: &mut BufReader<File>, file_size: u64, file_path: &Path) -> &'static str {
    if file_size < 8 {
        return "NOHIT";
    }

    reader.seek(SeekFrom::Start(0)).unwrap();
    let mut first_8_bytes = [0; 8];
    reader.read_exact(&mut first_8_bytes).unwrap();

    match first_8_bytes {
        [0x4D, ..] => match first_8_bytes {
            [0x4D, 0x5A, ..] => mz_file(reader, file_size),
            _ => NOHIT,
        },
        [0x50, ..] => match first_8_bytes {
            [0x50, 0x4B, 0x03, 0x04, ..] => zip_file(file_path),
            _ => NOHIT,
        },
        [0xD0, ..] => match first_8_bytes {
            [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1] => ms_compound_file(reader, file_size),
            _ => NOHIT,
        },
        _ => NOHIT,
    }
}

fn mz_file(reader: &mut BufReader<File>, file_size: u64) -> &'static str {
    match check_ms_exe(reader, file_size) {
        Ok(result) => match result.as_str() {
            EXE => EXE,
            DLL => DLL,
            SYS => SYS,
            EXE_LE_LX => EXE_LE_LX,
            EXE_NE => EXE_NE,
            EXE_MSDOS => EXE_MSDOS,
            _ => NOHIT,
        },
        _ => NOHIT,
    }
}

fn check_ms_exe(reader: &mut BufReader<File>, file_size: u64) -> Result<String, Box<dyn Error>> {
    //check for msdos exe
    if file_size < 32 {
        return Ok(NOHIT.to_string());
    }

    reader.seek(SeekFrom::Start(24))?;

    let mut reloc_table = [0; 2];
    reader.read_exact(&mut reloc_table)?;

    let reloc_table_offset = u16::from_le_bytes([reloc_table[0], reloc_table[1]]);

    if let 28..=63 = reloc_table_offset {
        return Ok(EXE_MSDOS.to_string());
    }

    // check for modern exe
    if file_size < 224 {
        return Ok(NOHIT.to_string());
    }

    reader.seek(SeekFrom::Start(60))?;

    let mut pe_header_addr = [0; 4];
    reader.read_exact(&mut pe_header_addr)?;

    let pe_addr_offset = u32::from_le_bytes([
        pe_header_addr[0],
        pe_header_addr[1],
        pe_header_addr[2],
        pe_header_addr[3],
    ]);

    reader.seek(SeekFrom::Start(pe_addr_offset as u64))?;

    let mut sig = [0; 2];
    reader.read_exact(&mut sig)?;

    if &sig == b"PE" {
        reader.seek(SeekFrom::Current(18))?;

        let mut size_optional_header = [0; 2];
        reader.read_exact(&mut size_optional_header)?;
        let size_of_optional_header = u16::from_le_bytes(size_optional_header);

        let mut file_characteristics = [0; 2];
        reader.read_exact(&mut file_characteristics)?;
        let file_characteristics = u16::from_le_bytes(file_characteristics);

        if file_characteristics & 0x2000 != 0 {
            Ok(DLL.to_string())
        } else if file_characteristics & 0x0002 != 0 {
            if size_of_optional_header == 0 {
                return Ok(NOHIT.to_string());
            } else {
                reader.seek(SeekFrom::Current(68))?;

                let mut subsystem = [0; 2];
                reader.read_exact(&mut subsystem)?;
                let subsystem = u16::from_le_bytes(subsystem);

                if subsystem == 0x01 {
                    return Ok(SYS.to_string());
                } else {
                    return Ok(EXE.to_string());
                }
            }
        } else {
            return Ok(NOHIT.to_string());
        }
    } else if &sig == b"NE" {
        return Ok(EXE_NE.to_string());
    } else if &sig == b"LE" || &sig == b"LX" {
        return Ok(EXE_LE_LX.to_string());
    } else {
        return Ok(NOHIT.to_string());
    }
}

fn zip_file(file_path: &Path) -> &'static str {
    let ms_openxml_file = "[Content_Types].xml";
    let mimetype_file = "mimetype";
    let apk_file = "AndroidManifest.xml";
    let xpi_file = "install.rdf";

    let files_to_find = [ms_openxml_file, mimetype_file, apk_file, xpi_file];

    let jar_file = "META-INF/MANIFEST.MF";
    let optional_files = [jar_file];

    let search = match utils::zip_find_files(file_path, &files_to_find, &optional_files) {
        Ok(files) => {
            for file in files.iter() {
                if file.contains(ms_openxml_file) {
                    return ms_openxml_format(file_path);
                }
                if file.contains(mimetype_file) {
                    return check_mimetype(file_path);
                }
                if file.contains(apk_file) {
                    return APK;
                }
                if file.contains(xpi_file) {
                    return XPI;
                }
                if file.contains(jar_file) {
                    return if files.len() == 1 {
                        JAR
                    } else {
                        continue;
                    };
                }
            }
            NOHIT
        }
        Err(_) => NOHIT,
    };

    if search != NOHIT {
        search
    } else {
        NOHIT
    }
}

fn ms_openxml_format(file_path: &Path) -> &'static str {
    match utils::zip_get_filecontent(file_path, "[Content_Types].xml") {
        Ok(content) => {
            if content.contains("/ppt/") {
                match content {
                    _ if content.contains(".addin.macroEnabled.") => PPAM,
                    _ if content.contains(".template.macroEnabled.") => POTM,
                    _ if content.contains(".slideshow.macroEnabled.") => PPSM,
                    _ if content.contains(".template.") => POTX,
                    _ if content.contains(".macroEnabled.") => PPTM,
                    _ if content.contains(".slideshow.") => PPSX,
                    _ => PPTX,
                }
            } else if content.contains("/word/") {
                match content {
                    _ if content.contains(".template.macroEnabled.") => DOTM,
                    _ if content.contains(".template.") => DOTX,
                    _ if content.contains(".macroEnabled.") => DOCM,
                    _ => DOCX,
                }
            } else if content.contains("/xl/") {
                match content {
                    _ if content.contains(".binary.") => XLSB,
                    _ if content.contains(".addin.macroEnabled.") => XLAM,
                    _ if content.contains(".template.macroEnabled.") => XLTM,
                    _ if content.contains(".template.") => XLTX,
                    _ if content.contains(".macroEnabled.") => XLSM,
                    _ => XLSX,
                }
            } else if content.contains("nuspec") {
                NUPKG
            } else if content.contains(".ms-package.xps") {
                XPS_OXPS
            } else {
                NOHIT
            }
        }
        Err(_) => NOHIT,
    }
}

fn check_mimetype(file_path: &Path) -> &'static str {
    match utils::zip_get_filecontent(file_path, "mimetype") {
        Ok(content) => {
            if content.ends_with("epub+zip") {
                EPUB
            } else if content.ends_with("vnd.sun.xml.writer") {
                SXW_STW
            } else if content.ends_with("vnd.sun.xml.calc") {
                SXC_STC
            } else if content.ends_with("vnd.sun.xml.impress") {
                SXI_STI
            } else if content.ends_with("vnd.sun.xml.math") {
                SXM
            } else if content.ends_with("vnd.sun.xml.draw") {
                SXD
            } else if content.ends_with("image/openraster") {
                ORA
            } else if content.starts_with("application/vnd.oasis.opendocument.") {
                match content {
                    _ if content.ends_with("text") => ODT,
                    _ if content.ends_with("spreadsheet") => ODS,
                    _ if content.ends_with("presentation") => ODP,
                    _ if content.ends_with("graphics") => ODG,
                    _ if content.ends_with("chart") => ODC,
                    _ if content.ends_with("formula") => ODF,
                    _ if content.ends_with("image") => ODI,
                    _ if content.ends_with("base") => ODB,
                    _ if content.ends_with("text-web") => OTH,
                    _ if content.ends_with("text-master") => ODM,
                    _ if content.ends_with("text-template") => OTT,
                    _ if content.ends_with("spreadsheet-template") => OTS,
                    _ if content.ends_with("presentation-template") => OTP,
                    _ if content.ends_with("graphics-template") => OTG,
                    _ if content.ends_with("chart-template") => OTC,
                    _ if content.ends_with("formula-template") => OTF,
                    _ if content.ends_with("image-template") => OTI,
                    _ => OD,
                }
            } else {
                NOHIT
            }
        }
        Err(_) => NOHIT,
    }
}

fn ms_compound_file(reader: &mut BufReader<File>, file_size: u64) -> &'static str {
    if file_size < 48 + 4 {
        return NOHIT;
    };

    reader.seek(SeekFrom::Start(48)).unwrap();
    let mut offset_48 = [0; 4];
    reader.read_exact(&mut offset_48).unwrap();

    let offset_48_val =
        u32::from_le_bytes([offset_48[0], offset_48[1], offset_48[2], offset_48[3]]);

    let root_entry = 512 * (1 + offset_48_val) as u64;
    let clsid_offset = root_entry + 80;

    if file_size < clsid_offset + 16 {
        return NOHIT;
    };

    reader.seek(SeekFrom::Start(clsid_offset)).unwrap();
    let mut clsid = [0; 4];
    reader.read_exact(&mut clsid).unwrap();

    match clsid {
        [0x00, 0x00, 0x00, 0x00] => {
            let next_entry = root_entry + 128;
            if file_size < next_entry + 32 {
                return NOHIT;
            }

            reader.seek(SeekFrom::Start(next_entry)).unwrap();
            let mut sig = [0; 32];
            reader.read_exact(&mut sig).unwrap();

            match sig {
                [0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ..] => MS_THUMBSDB,
                [0x57, 0x00, 0x6F, 0x00, 0x72, 0x00, 0x6B, 0x00, 0x62, 0x00, 0x6F, 0x00, 0x6F, 0x00, 0x6B, 0x00, ..] => {
                    XLS
                }
                _ => MS_COMPOUND_ZERO,
            }
        }
        [0x00, 0x04, 0x85, 0x59] => OBD,
        [0x00, 0x09, 0x02, 0x00] => DOC_DOT,
        [0x00, 0x12, 0x02, 0x00] => PUB,
        [0x00, 0x67, 0x61, 0x56] => FPX,
        [0x00, 0x68, 0x61, 0x56] => MIX,
        [0x01, 0x12, 0x02, 0x00] => PUB,
        [0x02, 0x13, 0x02, 0x00] => WPS_WPT,
        [0x03, 0x13, 0x02, 0x00] => WDB,
        [0x06, 0x09, 0x02, 0x00] => DOC_DOT,
        [0x0B, 0x0D, 0x02, 0x00] => MSG,
        [0x10, 0x08, 0x02, 0x00] => XLS,
        [0x10, 0x8D, 0x81, 0x64] => PPT_PPS,
        [0x13, 0x1A, 0x02, 0x00] => VSD,
        [0x14, 0x1A, 0x02, 0x00] => VSD,
        [0x20, 0x08, 0x02, 0x00] => XLS,
        [0x20, 0xF4, 0x14, 0xDE] => CCD_CCT,
        [0x21, 0x43, 0x88, 0xBF] => SDS,
        [0x21, 0x72, 0x5C, 0x56] => SDD,
        [0x21, 0xD0, 0xB8, 0x18] => MIX,
        [0x3A, 0x8F, 0xB7, 0x74] => MPP,
        [0x40, 0x7E, 0x5C, 0xDC] => SDW,
        [0x41, 0xD4, 0x61, 0x63] => SDC,
        [0x44, 0x2C, 0x02, 0x00] => DTP,
        [0x46, 0xF0, 0x06, 0x00] => MSG,
        [0x49, 0x4E, 0x4D, 0x4F] => OFM,
        [0x51, 0x48, 0x04, 0x00] => PPT_PPS,
        [0x57, 0x02, 0x00, 0x00] => FTM_FTW,
        [0x60, 0x04, 0x59, 0xD4] => SMF,
        [0x60, 0x2C, 0x02, 0x00] => ART,
        [0x60, 0xFE, 0x2E, 0x40] => WPG,
        [0x61, 0xB8, 0xA5, 0xC6] => SDC,
        [0x62, 0xFE, 0x2E, 0x40] => SHW,
        [0x70, 0xAA, 0x7C, 0x59] => FLA_SPA,
        [0x70, 0xAE, 0x7B, 0xEA] => PPT_PPS,
        [0x70, 0xC9, 0x0A, 0x34] => SGL,
        [0x7B, 0x8C, 0xDD, 0x1C] => MAX,
        [0x80, 0x1C, 0xB0, 0x02] => AWD,
        [0x82, 0x10, 0x0C, 0x00] => MSI,
        [0x84, 0x10, 0x0C, 0x00] => MSI,
        [0x84, 0x1F, 0x85, 0x31] => PTM,
        [0x86, 0x10, 0x0C, 0x00] => MSP,
        [0x90, 0xB4, 0x29, 0x4D] => IPT,
        [0xA0, 0x05, 0x89, 0x2E] => SDA,
        [0xA0, 0x3F, 0x54, 0x3F] => SDC,
        [0xB0, 0xE9, 0x04, 0x8B] => SDW,
        [0xB2, 0x5A, 0xA4, 0x0E] => WPS_WPT,
        [0xC0, 0x3C, 0x2D, 0x01] => SDD,
        [0xC0, 0xC7, 0x26, 0x6E] => WPF_WORKS,
        [0xD1, 0xF9, 0x0C, 0xC2] => SDW,
        [0xD3, 0xF9, 0x0C, 0xC2] => SGL,
        [0xE0, 0xB7, 0xB3, 0x02] => SDS,
        [0xE1, 0x63, 0x5E, 0xC6] => SCN,
        [0xE1, 0xB7, 0xB3, 0x02] => SMF,
        [0xC2, 0xDB, 0xCD, 0x28] => WPS_WPT,
        [0xC3, 0xDB, 0xCD, 0x28] => WDB,
        [0xE0, 0x99, 0x9C, 0xFB] => SDS,
        [0xE0, 0xAA, 0x10, 0xAF] => SDA,
        [0x40, 0xE6, 0xB5, 0xFF] => SMF,
        [0xF0, 0x46, 0x72, 0x81] => PPA_PWZ,
        [0xFF, 0x73, 0x98, 0x51] => WPD,
        _ => NOHIT,
    }
}
