use super::types::{Coin, DeepBookPackageIds, Pool};
use lazy_static;
use std::collections::HashMap;

pub const TESTNET_PACKAGE_IDS: DeepBookPackageIds = DeepBookPackageIds {
    deepbook_package_id: "0xa3886aaa8aa831572dd39549242ca004a438c3a55967af9f0387ad2b01595068",
    registry_id: "0x7c256edbda983a2cd6f946655f4bf3f00a41043993781f8674a7046e8c0e11d1",
    deep_treasury_id: "0x69fffdae0075f8f71f4fa793549c11079266910e8905169845af1f5d00e09dcb",
};

pub const MAINNET_PACKAGE_IDS: DeepBookPackageIds = DeepBookPackageIds {
    deepbook_package_id: "0xb29d83c26cdd2a64959263abbcfc4a6937f0c9fccaf98580ca56faded65be244",
    registry_id: "0xaf16199a2dff736e9f07a845f23c5da6df6f756eddb631aed9d24a93efc4549d",
    deep_treasury_id: "0x032abf8948dda67a271bcc18e776dbbcfb0d58c8d288a700ff0d5521e57a1ffe",
};

lazy_static::lazy_static! {
    pub static ref TESTNET_COINS: HashMap<&'static str, Coin> = {
        let mut m = HashMap::new();
        m.insert("DEEP", Coin {
            address: String::from("0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8"),
            type_name: String::from("0x36dbef866a1d62bf7328989a10fb2f07d769f4ee587c0de4a0a256e57e0a58a8::deep::DEEP"),
            scalar: 1_000_000,
        });
        m.insert("SUI", Coin {
            address: String::from("0x0000000000000000000000000000000000000000000000000000000000000002"),
            type_name: String::from("0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI"),
            scalar: 1_000_000_000,
        });
        m.insert("DBUSDC", Coin {
            address: String::from("0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7"),
            type_name: String::from("0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDC::DBUSDC"),
            scalar: 1_000_000,
        });
        m.insert("DBUSDT", Coin {
            address: String::from("0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7"),
            type_name: String::from("0xf7152c05930480cd740d7311b5b8b45c6f488e3a53a11c3f74a6fac36a52e0d7::DBUSDT::DBUSDT"),
            scalar: 1_000_000,
        });
        m
    };

    pub static ref MAINNET_COINS: HashMap<&'static str, Coin> = {
        let mut m = HashMap::new();
        m.insert("DEEP", Coin {
            address: String::from("0xdeeb7a4662eec9f2f3def03fb937a663dddaa2e215b8078a284d026b7946c270"),
            type_name: String::from("0xdeeb7a4662eec9f2f3def03fb937a663dddaa2e215b8078a284d026b7946c270::deep::DEEP"),
            scalar: 1_000_000,
        });
        m.insert("SUI", Coin {
            address: String::from("0x0000000000000000000000000000000000000000000000000000000000000002"),
            type_name: String::from("0x0000000000000000000000000000000000000000000000000000000000000002::sui::SUI"),
            scalar: 1_000_000_000,
        });
        m.insert("USDC", Coin {
            address: String::from("0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7"),
            type_name: String::from("0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7::usdc::USDC"),
            scalar: 1_000_000,
        });
        m.insert("WUSDC", Coin {
            address: String::from("0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf"),
            type_name: String::from("0x5d4b302506645c37ff133b98c4b50a5ae14841659738d6d733d59d0d217a93bf::coin::COIN"),
            scalar: 1_000_000,
        });
        m.insert("WETH", Coin {
            address: String::from("0xaf8cd5edc19c4512f4259f0bee101a40d41ebed738ade5874359610ef8eeced5"),
            type_name: String::from("0xaf8cd5edc19c4512f4259f0bee101a40d41ebed738ade5874359610ef8eeced5::coin::COIN"),
            scalar: 100_000_000,
        });
        m.insert("BETH", Coin {
            address: String::from("0xd0e89b2af5e4910726fbcd8b8dd37bb79b29e5f83f7491bca830e94f7f226d29"),
            type_name: String::from("0xd0e89b2af5e4910726fbcd8b8dd37bb79b29e5f83f7491bca830e94f7f226d29::eth::ETH"),
            scalar: 100_000_000,
        });
        m.insert("WBTC", Coin {
            address: String::from("0x027792d9fed7f9844eb4839566001bb6f6cb4804f66aa2da6fe1ee242d896881"),
            type_name: String::from("0x027792d9fed7f9844eb4839566001bb6f6cb4804f66aa2da6fe1ee242d896881::coin::COIN"),
            scalar: 100_000_000,
        });
        m.insert("WUSDT", Coin {
            address: String::from("0xc060006111016b8a020ad5b33834984a437aaa7d3c74c18e09a95d48aceab08c"),
            type_name: String::from("0xc060006111016b8a020ad5b33834984a437aaa7d3c74c18e09a95d48aceab08c::coin::COIN"),
            scalar: 1_000_000,
        });
        m.insert("NS", Coin {
            address: String::from("0x5145494a5f5100e645e4b0aa950fa6b68f614e8c59e17bc5ded3495123a79178"),
            type_name: String::from("0x5145494a5f5100e645e4b0aa950fa6b68f614e8c59e17bc5ded3495123a79178::ns::NS"),
            scalar: 1_000_000,
        });
        m.insert("TYPUS", Coin {
            address: String::from("0xf82dc05634970553615eef6112a1ac4fb7bf10272bf6cbe0f80ef44a6c489385"),
            type_name: String::from("0xf82dc05634970553615eef6112a1ac4fb7bf10272bf6cbe0f80ef44a6c489385::typus::TYPUS"),
            scalar: 1_000_000_000,
        });
        m.insert("AUSD", Coin {
            address: String::from("0x2053d08c1e2bd02791056171aab0fd12bd7cd7efad2ab8f6b9c8902f14df2ff2"),
            type_name: String::from("0x2053d08c1e2bd02791056171aab0fd12bd7cd7efad2ab8f6b9c8902f14df2ff2::ausd::AUSD"),
            scalar: 1_000_000,
        });
        m.insert("DRF", Coin {
            address: String::from("0x294de7579d55c110a00a7c4946e09a1b5cbeca2592fbb83fd7bfacba3cfeaf0e"),
            type_name: String::from("0x294de7579d55c110a00a7c4946e09a1b5cbeca2592fbb83fd7bfacba3cfeaf0e::drf::DRF"),
            scalar: 1_000_000,
        });
        m.insert("SEND", Coin {
            address: String::from("0xb45fcfcc2cc07ce0702cc2d229621e046c906ef14d9b25e8e4d25f6e8763fef7"),
            type_name: String::from("0xb45fcfcc2cc07ce0702cc2d229621e046c906ef14d9b25e8e4d25f6e8763fef7::send::SEND"),
            scalar: 1_000_000,
        });
        m.insert("WAL", Coin {
            address: String::from("0x356a26eb9e012a68958082340d4c4116e7f55615cf27affcff209cf0ae544f59"),
            type_name: String::from("0x356a26eb9e012a68958082340d4c4116e7f55615cf27affcff209cf0ae544f59::wal::WAL"),
            scalar: 1_000_000_000,
        });
        m.insert("XBTC", Coin {
            address: String::from("0x876a4b7bce8aeaef60464c11f4026903e9afacab79b9b142686158aa86560b50"),
            type_name: String::from("0x876a4b7bce8aeaef60464c11f4026903e9afacab79b9b142686158aa86560b50::xbtc::XBTC"),
            scalar: 100_000_000,
        });
        m.insert("IKA", Coin {
            address: String::from("0x7262fb2f7a3a14c888c438a3cd9b912469a58cf60f367352c46584262e8299aa"),
            type_name: String::from("0x7262fb2f7a3a14c888c438a3cd9b912469a58cf60f367352c46584262e8299aa::ika::IKA"),
            scalar: 1_000_000_000,
        });
        // This coin is experimental
        m.insert("WGIGA", Coin {
            address: String::from("0xec32640add6d02a1d5f0425d72705eb76d9de7edfd4f34e0dba68e62ecceb05b"),
            type_name: String::from("0xec32640add6d02a1d5f0425d72705eb76d9de7edfd4f34e0dba68e62ecceb05b::coin::COIN"),
            scalar: 100_000,
        });
        m
    };

    pub static ref TESTNET_POOLS: HashMap<&'static str, Pool> = {
        let mut m = HashMap::new();
        m.insert("DEEP_SUI", Pool {
            address: String::from("0x48c95963e9eac37a316b7ae04a0deb761bcdcc2b67912374d6036e7f0e9bae9f"),
            base_coin: String::from("DEEP"),
            quote_coin: String::from("SUI"),
        });
        m.insert("SUI_DBUSDC", Pool {
            address: String::from("0x1c19362ca52b8ffd7a33cee805a67d40f31e6ba303753fd3a4cfdfacea7163a5"),
            base_coin: String::from("SUI"),
            quote_coin: String::from("DBUSDC"),
        });
        m.insert("DEEP_DBUSDC", Pool {
            address: String::from("0xe86b991f8632217505fd859445f9803967ac84a9d4a1219065bf191fcb74b622"),
            base_coin: String::from("DEEP"),
            quote_coin: String::from("DBUSDC"),
        });
        m.insert("DBUSDT_DBUSDC", Pool {
            address: String::from("0x83970bb02e3636efdff8c141ab06af5e3c9a22e2f74d7f02a9c3430d0d10c1ca"),
            base_coin: String::from("DBUSDT"),
            quote_coin: String::from("DBUSDC"),
        });
        m.insert("WAL_DBUSDC", Pool {
            address: String::from("0xeb524b6aea0ec4b494878582e0b78924208339d360b62aec4a8ecd4031520dbb"),
            base_coin: String::from("WAL"),
            quote_coin: String::from("WAL"),
        });
        m.insert("WAL_SUI", Pool {
            address: String::from("0x8c1c1b186c4fddab1ebd53e0895a36c1d1b3b9a77cd34e607bef49a38af0150a"),
            base_coin: String::from("WAL"),
            quote_coin: String::from("SUI"),
        });
        m
    };

    pub static ref MAINNET_POOLS: HashMap<&'static str, Pool> = {
        let mut m = HashMap::new();
        m.insert("DEEP_SUI", Pool {
            address: String::from("0xb663828d6217467c8a1838a03793da896cbe745b150ebd57d82f814ca579fc22"),
            base_coin: String::from("DEEP"),
            quote_coin: String::from("SUI"),
        });
        m.insert("SUI_USDC", Pool {
            address: String::from("0xe05dafb5133bcffb8d59f4e12465dc0e9faeaa05e3e342a08fe135800e3e4407"),
            base_coin: String::from("SUI"),
            quote_coin: String::from("USDC"),
        });
        m.insert("DEEP_USDC", Pool {
            address: String::from("0xf948981b806057580f91622417534f491da5f61aeaf33d0ed8e69fd5691c95ce"),
            base_coin: String::from("DEEP"),
            quote_coin: String::from("USDC"),
        });
        m.insert("WUSDT_USDC", Pool {
            address: String::from("0x4e2ca3988246e1d50b9bf209abb9c1cbfec65bd95afdacc620a36c67bdb8452f"),
            base_coin: String::from("WUSDT"),
            quote_coin: String::from("USDC"),
        });
        m.insert("WUSDC_USDC", Pool {
            address: String::from("0xa0b9ebefb38c963fd115f52d71fa64501b79d1adcb5270563f92ce0442376545"),
            base_coin: String::from("WUSDC"),
            quote_coin: String::from("USDC"),
        });
        m.insert("BETH_USDC", Pool {
            address: String::from("0x1109352b9112717bd2a7c3eb9a416fff1ba6951760f5bdd5424cf5e4e5b3e65c"),
            base_coin: String::from("BETH"),
            quote_coin: String::from("USDC"),
        });
        m.insert("NS_USDC", Pool {
            address: String::from("0x0c0fdd4008740d81a8a7d4281322aee71a1b62c449eb5b142656753d89ebc060"),
            base_coin: String::from("NS"),
            quote_coin: String::from("USDC"),
        });
        m.insert("NS_SUI", Pool {
            address: String::from("0x27c4fdb3b846aa3ae4a65ef5127a309aa3c1f466671471a806d8912a18b253e8"),
            base_coin: String::from("NS"),
            quote_coin: String::from("SUI"),
        });
        m.insert("TYPUS_SUI", Pool {
            address: String::from("0xe8e56f377ab5a261449b92ac42c8ddaacd5671e9fec2179d7933dd1a91200eec"),
            base_coin: String::from("TYPUS"),
            quote_coin: String::from("SUI"),
        });
        m.insert("SUI_AUSD", Pool {
            address: String::from("0x183df694ebc852a5f90a959f0f563b82ac9691e42357e9a9fe961d71a1b809c8"),
            base_coin: String::from("SUI"),
            quote_coin: String::from("AUSD"),
        });
        m.insert("AUSD_USDC", Pool {
            address: String::from("0x5661fc7f88fbeb8cb881150a810758cf13700bb4e1f31274a244581b37c303c3"),
            base_coin: String::from("AUSD"),
            quote_coin: String::from("USDC"),
        });
        m.insert("DRF_SUI", Pool {
            address: String::from("0x126865a0197d6ab44bfd15fd052da6db92fd2eb831ff9663451bbfa1219e2af2"),
            base_coin: String::from("DRF"),
            quote_coin: String::from("SUI"),
        });
        m.insert("SEND_USDC", Pool {
            address: String::from("0x1fe7b99c28ded39774f37327b509d58e2be7fff94899c06d22b407496a6fa990"),
            base_coin: String::from("SEND"),
            quote_coin: String::from("USDC"),
        });
        m.insert("WAL_USDC", Pool {
            address: String::from("0x56a1c985c1f1123181d6b881714793689321ba24301b3585eec427436eb1c76d"),
            base_coin: String::from("WAL"),
            quote_coin: String::from("USDC"),
        });
        m.insert("WAL_SUI", Pool {
            address: String::from("0x81f5339934c83ea19dd6bcc75c52e83509629a5f71d3257428c2ce47cc94d08b"),
            base_coin: String::from("WAL"),
            quote_coin: String::from("SUI"),
        });
        m.insert("XBTC_USDC", Pool {
            address: String::from("0x20b9a3ec7a02d4f344aa1ebc5774b7b0ccafa9a5d76230662fdc0300bb215307"),
            base_coin: String::from("XBTC"),
            quote_coin: String::from("USDC"),
        });
        m.insert("IKA_USDC", Pool {
            address: String::from("0xfa732993af2b60d04d7049511f801e79426b2b6a5103e22769c0cead982b0f47"),
            base_coin: String::from("IKA"),
            quote_coin: String::from("USDC"),
        });
        m
    };
}
