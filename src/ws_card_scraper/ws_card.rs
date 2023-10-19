#[derive(Debug, Clone, Eq, PartialEq)]
    pub(crate) enum WSCardType {
        Character,
        Event,
        Climax,
    }

    impl Default for WSCardType {
        fn default() -> Self {
            WSCardType::Character
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub(crate) enum WSCardSide {
        Weiß,
        Schwarz,
    }

    impl Default for WSCardSide {
        fn default() -> Self {
            WSCardSide::Weiß
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub(crate) enum WSCardColor {
        Red,
        Blue,
        Green,
        Yellow,
        Purple,
        Colorless,
    }

    impl Default for WSCardColor {
        fn default() -> Self {
            WSCardColor::Colorless
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub(crate) enum WSCardTrigger {
        None,
        Soul,
        DoubleSoul,
        Pool,
        Comeback,
        Return,
        Draw,
        Treasure,
        Shot,
        Gate,
        Choice,
        Standby,
    }

    impl Default for WSCardTrigger {
        fn default() -> Self {
            WSCardTrigger::None
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, Default)]
    pub(crate) struct Card {
        pub image: String,
        pub card_name: String,
        pub card_name_kana: String,
        pub card_no: String,
        pub product: String,
        pub expansion: String,
        pub expansion_id: String,
        pub rarity: String,
        pub side: WSCardSide,
        pub card_type: WSCardType,
        pub color: WSCardColor,
        pub level: u16,
        pub cost: u16,
        pub power: u16,
        pub soul: u8,
        pub trigger: WSCardTrigger,
        pub special_attribute: Vec<String>,
        pub text: String,
        pub flavor_text: String,
        pub illustrator: String,
    }
