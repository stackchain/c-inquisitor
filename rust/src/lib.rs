use js_error::JsError;
use crate::bingen::wasm_bindgen;

mod js_error;
pub mod csl_decoders;
pub mod plutus;
mod koios_client;
mod cbor;
mod netwrok_type;
mod bingen;

use crate::cbor::cbor_decoder::{fromhex_to_js_error, get_tokenizer, get_value};


#[wasm_bindgen]
pub fn cbor_to_json(cbor_hex: &str) -> Result<String, JsError> {
    let cbor = hex::decode(cbor_hex).map_err(fromhex_to_js_error)?;
    let tokenizer = get_tokenizer(&cbor);
    Ok(get_value(tokenizer)?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tx = hex::decode("84a90081825820636dd75b50cd87dbf3d3fc9ef16090a05c9f1533dabead98d4d7e247056b6ad6020183825839001ec4c4f8dc23885ab589decbc0aa4bd613574d322ddfdce646a976936589ce0980a92196fc44167e7fcf523c6be3ca14f89ee1b6a2ef57a7821a00126a10a1581c55ac682bcdd54bf58a3cc5149213be3385a2a88381a6bcc350679f6ca154000de14054616e676f43495030303638233030390183581d70e363cdc66d72bd67b6edf431a4e527b000d9f78e2b13d1749be801c5821a0012cf14a1581c55ac682bcdd54bf58a3cc5149213be3385a2a88381a6bcc350679f6ca154000643b054616e676f4349503030363823303039015820aa2aab9a728dbbcdb8eb1cbc397301ccee739e1bd5e214d815d4d2e363079a1d825839001ec4c4f8dc23885ab589decbc0aa4bd613574d322ddfdce646a976936589ce0980a92196fc44167e7fcf523c6be3ca14f89ee1b6a2ef57a71a0d91e0e1021a000d5ef5031a018b13ea080009a1581c55ac682bcdd54bf58a3cc5149213be3385a2a88381a6bcc350679f6ca254000643b054616e676f43495030303638233030390154000de14054616e676f4349503030363823303039010b5820b390f12bea7203a3bb6b992c6396f4086e8bc33e9a3114e3eb26f52ca4f07c320d81825820330d5c07cb6daa28cbe53b23290213c054f8987fdccf6bef266cb3d85287c7ad070e81581c1aabe8cdb1153e11c3363270fd11baef2ca758e56d9a0866e73f7dc5a50082825820cf76399a210de8720e9fa894e45e41e29ab525e30bc402801c076250d1585bcd5840af5bf4a8a0ee1b7bd1f9aff4c6cd544a69ed204cee0353d46f98510328dd41546062918d248c968f73cacde65370d9fde2667d8cd63860446239f029e0fb850a825820cf76399a210de8720e9fa894e45e41e29ab525e30bc402801c076250d1585bcd5840af5bf4a8a0ee1b7bd1f9aff4c6cd544a69ed204cee0353d46f98510328dd41546062918d248c968f73cacde65370d9fde2667d8cd63860446239f029e0fb850a0380068159130b5913080100003333323232332233223232323232323232323232323232323322323232323232323232333222323232323232323232323232323232323322323232322223232323232232232232325335330063333573466e1d401120042122200223333573466e1d401520022122200323333573466e1d401920002122200123263204233573808608408007e07c6666ae68cdc39aab9d5002480008cc8848cc00400c008c8c8c8c8c8c8c8c8c8c8c8c8c8cccd5cd19b8735573aa018900011999999999999111111111110919999999999980080680600580500480400380300280200180119a81d81e1aba1500c33503b03c35742a01666a07607a6ae854028ccd540fdd7281f1aba150093335503f75ca07c6ae854020cd40ec118d5d0a803999aa81f823bad35742a00c6464646666ae68cdc39aab9d5002480008cc8848cc00400c008c8c8c8cccd5cd19b8735573aa004900011991091980080180119a828bad35742a00460a66ae84d5d1280111931902b19ab9c057056054135573ca00226ea8004d5d0a8011919191999ab9a3370e6aae7540092000233502833505175a6ae854008c14cd5d09aba250022326320563357380ae0ac0a826aae7940044dd50009aba135744a004464c640a466ae7014c1481404d55cf280089baa00135742a00a66a076eb8d5d0a802199aa81f82190009aba150033335503f75c40026ae854008c114d5d09aba2500223263204e33573809e09c09826ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aba25001135744a00226ae8940044d5d1280089aab9e5001137540026ae854008c0d4d5d09aba2500223263204033573808208007c207e264c6407e66ae712401035054350003f135573ca00226ea80044d55ce9baa001323232322232323253335005133355301b120013502d502c23500122253353302300350061333573466e1c005200104b04a104b30223550032222222222220081047153355335323235009222225333333500513006333302e005004003002153350012133355302312001350355034007001104d153350012133355302312001350215026007001104d1533500121533500321333573466e20cccd54c09048004c8cd40e088ccd40a400c004008d4098004cd40dc888c00cc0080048004894cd4c0280084cc0ac005200210010024800000413c140413841344c018cccc0b801401000c0084c018cccc0b801401000c0088d400488894cccccd401054cd400884c8c8d400488d4008894ccd4ccd40c8018d412c01c00854cd4004414c4150414c4150d402088888888888801540bc413041304130413054cd400884d4d401c88888888888801488c8c8d400488d4014894ccd4ccd40d001801000854cd400c54cd400441544158415441584154cd4078d412001014540c4413054cd400c854cd4d401c8888888888894cd4ccd54c0b848004cd4104894cd40088400c4005412094cd4ccd5cd19b8f00e00105b05a13504a001150490042105b1059104e13357389210e696e76616c6964207369676e65720004d104c50031047133573892010e696e76616c696420706f6c69637900046133001323500122222222222233355302312001223500222223500422335002200825335333573466e3c0040701741704cd4178cd5418001401802040214158029400cc088d5400c8888888888880204118c8004d5412c8894cd400854cd40044124884128884d400888c94ccd400c4134841388c854cd4020413c884d40088894cd40104150884d40088894cd54cd54cd4c8d4004888888888888c06000940588416c416441684cd5ce2490d696e76616c696420646174756d000591533553355335333573466e1cd5403488005200205a05915335333573466e1c019200205a05915335333573466e1c005200205a05915335333573466e3d4054d540348800816816454cd4cc0c940540204cc0c9405400c4164416441644164416441684cd5ce2491d696e76616c696420706f6c6963792069642c207175616e74697469657300059153355335333573466e3cccdc601a19b81371a00e06800e666e300d0cdc09b8d00203400205a059105a13357389210d696e76616c696420434950363800059153355335333573466e3ccc0980d001c07016816441684cd5ce19b964910d696e76616c6964203232323a20003732666e300d0cdc09b8d007034007059153355335333573466e3ccc0980d000806c16816441684cd5ce24810b696e76616c6964203130300005913301401000410591059105910591059320013550542253350011503f22135002225335333573466e3c00801c1541504d41100044c01800c54cd4c0a0c8cd54c084480048d400488cd54150008cd54c090480048d400488cd5415c008ccd40048cc0b92000001223302f00200123302e00148000004cd54c084480048d400488cd54150008ccd40048cd54c094480048d400488cd54160008d5409800400488ccd5540840ac0080048cd54c094480048d400488cd54160008d54094004004ccd554070098008004c8cd54c084480048d400488cd54150008cd54c090480048d400488cd5415c008cdc12400200200200266a09c66aa0a00a266a09c66aa0a00a26094002a09ea09e2c4426a004444a66a008266aa0aa006002442c26a004446666a002464c6408666ae712401024c680004320012326320433357389201024c68000432326320433357389201024c680004313500122002375c0086666ae68cdc39aab9d5008480008ccccc8888848ccccc00401801401000c008c0e8d5d0a80419a8173ae35742a00e66a05ceb4d5d0a80319a8173ad35742a00a66a05c6a0524646464646666ae68cdc39aab9d5004480008cccc09cc100d5d0a80219a81a3ae35742a00666a068eb4d5d0a80119a81a3ad357426ae8940088c98c8108cd5ce02182102009aba25001135744a00226aae7940044dd50009aba135744a00a464c6407866ae700f40f00e8dd70049bae009135744a00226ae8940044d5d1280089aab9e50011375400244666e3120000020011221233001003002111222333553004120015036335530081200123500122335503b00235500900133355300412001223500222533533355300d120013500b5010235001223300a0020050061003133503a004003503700133553008120012350012232335503c00330010053200135503d225335001135500a003221350022253353300c002008112223300200a0041300600300232001355036221122253350011002221330050023335530071200100500400111212223003004112122230010041233501622333500322002002001350012200132001355032221122533500115033221335034300400233553006120010040013200135503122112225335001135006003221333500900530040023335530071200100500400112350012200112350012200211233001225335002102c100102922333573466e3c0080040a80a488cdc0001000a4010466004a050002640026aa052444a66a00220044426a004446600e66601000400c002006640026aa0504444a66a00220044426a00444a66a666ae68cdc3800a4000056054266601000e00c006266601000e66a0582466600201000600400c006244464646464a666a00c42a666a00c42a666a0104260089309801a4c2a666a00e4260089309801a4c201e201a2a666a00e4260089309801a4c2a666a00c4260089309801a4c201c2a666a00a42018201a20162a666a00a42a666a00e42600a930980224c2a666a00c42600a930980224c201c20182a666a00c42600a930980224c2a666a00a42600a930980224c201a4a666a00a42a666a00e42a666a00e42666a0160140040022c2c2c201a2a666a00c42a666a00c42666a0140120040022c2c2c201820164a666a00842a666a00c42a666a00c42666a0140120040022c2c2c20182a666a00a42a666a00a42666a0120100040022c2c2c201620144a666a00642a666a00a42a666a00a42666a0120100040022c2c2c20162a666a00842a666a00842666a01000e0040022c2c2c201420124a666a00442a666a00842a666a00842666a01000e0040022c2c2c20142a666a00642a666a00642666a00e00c0040022c2c2c20122010246a0024444444400e444424666600200a0080060042246666666600244666ae68cdc380100081101091299a999ab9a3370e004002044042200c2a66a666ae68cdc48010008110108802080291199ab9a3371000400204404244666ae68cdc480100081101091199ab9a3371200400204204444666ae68cdc400100081081111299a999ab9a337120040020440422002200444a66a666ae68cdc4801000811010880108008911001891100109110008891980091299a8010800880e80e0919a80111199a801910010010009a8009100089109198008018010919118011bac0013200135501e2233335573e0024a03c466a03a60086ae84008c00cd5d100100b119191999ab9a3370e6aae7540092000233221233001003002300c35742a004600a6ae84d5d1280111931900b19ab9c017016014135573ca00226ea80048c8c8c8c8cccd5cd19b8735573aa00890001199991110919998008028020018011919191999ab9a3370e6aae7540092000233221233001003002301535742a00466a01a0286ae84d5d1280111931900d99ab9c01c01b019135573ca00226ea8004d5d0a802199aa8043ae500735742a0066464646666ae68cdc3a800a4008464244460040086ae84d55cf280191999ab9a3370ea0049001119091118008021bae357426aae7940108cccd5cd19b875003480008488800c8c98c8074cd5ce00f00e80d80d00c89aab9d5001137540026ae854008cd4025d71aba135744a004464c6402e66ae7006005c0544d5d1280089aba25001135573ca00226ea80044cd54005d73ad112232230023756002640026aa03644646666aae7c008940708cd406ccd54074c018d55cea80118029aab9e500230043574400602826ae84004488c8c8cccd5cd19b875001480008d401cc014d5d09aab9e500323333573466e1d400920022500723263201433573802a02802402226aae7540044dd50008909118010018891000919191999ab9a3370ea002900311909111180200298039aba135573ca00646666ae68cdc3a8012400846424444600400a60126ae84d55cf280211999ab9a3370ea006900111909111180080298039aba135573ca00a46666ae68cdc3a8022400046424444600600a6eb8d5d09aab9e500623263201233573802602402001e01c01a26aae7540044dd5000919191999ab9a3370e6aae7540092000233221233001003002300535742a0046eb4d5d09aba2500223263200e33573801e01c01826aae7940044dd50009191999ab9a3370e6aae75400520002375c6ae84d55cf280111931900619ab9c00d00c00a13754002464646464646666ae68cdc3a800a401842444444400646666ae68cdc3a8012401442444444400846666ae68cdc3a801a40104664424444444660020120106eb8d5d0a8029bad357426ae8940148cccd5cd19b875004480188cc8848888888cc008024020dd71aba15007375c6ae84d5d1280391999ab9a3370ea00a900211991091111111980300480418061aba15009375c6ae84d5d1280491999ab9a3370ea00c900111909111111180380418069aba135573ca01646666ae68cdc3a803a400046424444444600a010601c6ae84d55cf280611931900a99ab9c01601501301201101000f00e00d135573aa00826aae79400c4d55cf280109aab9e5001137540024646464646666ae68cdc3a800a4004466644424466600200a0080066eb4d5d0a8021bad35742a0066eb4d5d09aba2500323333573466e1d4009200023212230020033008357426aae7940188c98c8038cd5ce00780700600589aab9d5003135744a00226aae7940044dd5000919191999ab9a3370ea002900111909118008019bae357426aae79400c8cccd5cd19b875002480008c8488c00800cdd71aba135573ca008464c6401666ae7003002c0240204d55cea80089baa00112232323333573466e1d400520042122200123333573466e1d400920022350073006357426aae7940108cccd5cd19b87500348000848880088c98c8030cd5ce00680600500480409aab9d50011375400224244460060084646666ae68cdc3a800a4004401646666ae68cdc3a801240004016464c6400e66ae7002001c0140104d55ce9baa0012323333573466e1d4005200a2122222200223333573466e1d400920082122222200623333573466e1d400d20062122222200323333573466e1d401120042122222200423333573466e1d401520022122222200523333573466e1d401920002122222200123263200a33573801601401000e00c00a00800626aae74dd5000a4c240029201035054310032001355008223350014800088d4008894cd4ccd5cd19b8f00200d009008130070011300600332001355007223350014800088d4008894cd4ccd5cd19b8f00200c0080071001130060031220021220011122002122122330010040031122123300100300248900112323001001223300330020020014c10544000de140004c010544000643b0004c0132d8799fd87980d8799f581c1aabe8cdb1153e11c3363270fd11baef2ca758e56d9a0866e73f7dc5ffd87a80d87a80d87a80ff004c011e581ce363cdc66d72bd67b6edf431a4e527b000d9f78e2b13d1749be801c50001049fd8799fa158383535616336383262636464353462663538613363633531343932313362653333383561326138383338316136626363333530363739663663a15054616e676f4349503030363823303039a7446e616d655054616e676f434950303036382330303945696d6167655835697066733a2f2f516d53696e53595559526f3841437443583572636a526864413961476631564d734a413531514b677a326e57476449636f707972696768745054616e676f63727970746f2032303233466172746973744b54616e676f63727970746f496d656469615479706543706e674a636f6c6c656374696f6e5654616e676f63727970746f20436f6c6c656374696f6e4774776974746572582068747470733a2f2f747769747465722e636f6d2f74616e676f5f63727970746f01ffff0581840100d8799f474d696e744e4654ff821a0024f48f1a283ad9d3f5f6").unwrap();
        let tokenizer = get_tokenizer(&tx);
        let value = get_value(tokenizer).ok().unwrap();
        let str = value.to_string();
        println!("{:?}", str);
    }
}
