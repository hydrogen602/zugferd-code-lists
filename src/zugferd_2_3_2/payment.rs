#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Payment {
    InstrumentNotDefined,
    AutomatedClearingHouseCredit,
    AutomatedClearingHouseDebit,
    AchDemandDebitReversal,
    AchDemandCreditReversal,
    AchDemandCredit,
    AchDemandDebit,
    Hold,
    NationalOrRegionalClearing,
    InCash,
    AchSavingsCreditReversal,
    AchSavingsDebitReversal,
    AchSavingsCredit,
    AchSavingsDebit,
    BookentryCredit,
    BookentryDebit,
    AchDemandCashConcentrationDisbursementCcdCredit,
    AchDemandCashConcentrationDisbursementCcdDebit,
    AchDemandCorporateTradePaymentCtpCredit,
    Cheque,
    BankersDraft,
    CertifiedBankersDraft,
    BankChequeIssuedByABankingOrSimilarEstablishment,
    BillExchangeAwaitingAcceptance,
    CertifiedCheque,
    LocalCheque,
    AchDemandCorporateTradePaymentCtpDebit,
    AchDemandCorporateTradeExchangeCtxCredit,
    AchDemandCorporateTradeExchangeCtxDebit,
    CreditTransfer,
    DebitTransfer,
    AchDemandCashConcentrationDisbursementPlusCcd,
    AchDemandCashConcentrationDisbursementPlusCcd_Dup,
    AchPrearrangedPaymentAndDepositPpd,
    AchSavingsCashConcentrationDisbursementCcdCredit,
    AchSavingsCashConcentrationDisbursementCcdDebit,
    AchSavingsCorporateTradePaymentCtpCredit,
    AchSavingsCorporateTradePaymentCtpDebit,
    AchSavingsCorporateTradeExchangeCtxCredit,
    AchSavingsCorporateTradeExchangeCtxDebit,
    AchSavingsCashConcentrationDisbursementPlusCcd,
    PaymentToBankAccount,
    AchSavingsCashConcentrationDisbursementPlusCcd_Dup,
    AcceptedBillExchange,
    ReferencedHomeBankingCreditTransfer,
    InterbankDebitTransfer,
    HomeBankingDebitTransfer,
    BankCard,
    DirectDebit,
    PaymentByPostgiro,
    FrNorme697TelereglementCfonbFrenchOrganisationFor,
    UrgentCommercialPayment,
    UrgentTreasuryPayment,
    CreditCard,
    DebitCard,
    Bankgiro,
    StandingAgreement,
    SepaCreditTransfer,
    SepaDirectDebit,
    PromissoryNote,
    PromissoryNoteSignedByDebtor,
    PromissoryNoteSignedByDebtorAndEndorsedByABank,
    PromissoryNoteSignedByDebtorAndEndorsedByA,
    PromissoryNoteSignedByABank,
    PromissoryNoteSignedByABankAndEndorsedByAnother,
    PromissoryNoteSignedByAThirdParty,
    PromissoryNoteSignedByAThirdPartyAndEndorsedByA,
    OnlinePaymentService,
    TransferAdvice,
    BillDrawnByCreditorOnDebtor,
    BillDrawnByCreditorOnABank,
    BillDrawnByCreditorEndorsedByAnotherBank,
    BillDrawnByCreditorOnABankAndEndorsedByA,
    BillDrawnByCreditorOnAThirdParty,
    BillDrawnByCreditorOnThirdPartyAcceptedAnd,
    NotTransferableBankersDraft,
    NotTransferableLocalCheque,
    ReferenceGiro,
    UrgentGiro,
    FreeFormatGiro,
    RequestedMethodForPaymentWasNotUsed,
    ClearingBetweenPartners,
    JpElectronicallyRecordedMonetaryClaims,
    MutuallyDefined,
}

impl crate::Code for Payment {
    fn code(&self) -> &str {
        match self {
            Payment::InstrumentNotDefined => "1",
            Payment::AutomatedClearingHouseCredit => "2",
            Payment::AutomatedClearingHouseDebit => "3",
            Payment::AchDemandDebitReversal => "4",
            Payment::AchDemandCreditReversal => "5",
            Payment::AchDemandCredit => "6",
            Payment::AchDemandDebit => "7",
            Payment::Hold => "8",
            Payment::NationalOrRegionalClearing => "9",
            Payment::InCash => "10",
            Payment::AchSavingsCreditReversal => "11",
            Payment::AchSavingsDebitReversal => "12",
            Payment::AchSavingsCredit => "13",
            Payment::AchSavingsDebit => "14",
            Payment::BookentryCredit => "15",
            Payment::BookentryDebit => "16",
            Payment::AchDemandCashConcentrationDisbursementCcdCredit => "17",
            Payment::AchDemandCashConcentrationDisbursementCcdDebit => "18",
            Payment::AchDemandCorporateTradePaymentCtpCredit => "19",
            Payment::Cheque => "20",
            Payment::BankersDraft => "21",
            Payment::CertifiedBankersDraft => "22",
            Payment::BankChequeIssuedByABankingOrSimilarEstablishment => "23",
            Payment::BillExchangeAwaitingAcceptance => "24",
            Payment::CertifiedCheque => "25",
            Payment::LocalCheque => "26",
            Payment::AchDemandCorporateTradePaymentCtpDebit => "27",
            Payment::AchDemandCorporateTradeExchangeCtxCredit => "28",
            Payment::AchDemandCorporateTradeExchangeCtxDebit => "29",
            Payment::CreditTransfer => "30",
            Payment::DebitTransfer => "31",
            Payment::AchDemandCashConcentrationDisbursementPlusCcd => "32",
            Payment::AchDemandCashConcentrationDisbursementPlusCcd_Dup => "33",
            Payment::AchPrearrangedPaymentAndDepositPpd => "34",
            Payment::AchSavingsCashConcentrationDisbursementCcdCredit => "35",
            Payment::AchSavingsCashConcentrationDisbursementCcdDebit => "36",
            Payment::AchSavingsCorporateTradePaymentCtpCredit => "37",
            Payment::AchSavingsCorporateTradePaymentCtpDebit => "38",
            Payment::AchSavingsCorporateTradeExchangeCtxCredit => "39",
            Payment::AchSavingsCorporateTradeExchangeCtxDebit => "40",
            Payment::AchSavingsCashConcentrationDisbursementPlusCcd => "41",
            Payment::PaymentToBankAccount => "42",
            Payment::AchSavingsCashConcentrationDisbursementPlusCcd_Dup => "43",
            Payment::AcceptedBillExchange => "44",
            Payment::ReferencedHomeBankingCreditTransfer => "45",
            Payment::InterbankDebitTransfer => "46",
            Payment::HomeBankingDebitTransfer => "47",
            Payment::BankCard => "48",
            Payment::DirectDebit => "49",
            Payment::PaymentByPostgiro => "50",
            Payment::FrNorme697TelereglementCfonbFrenchOrganisationFor => "51",
            Payment::UrgentCommercialPayment => "52",
            Payment::UrgentTreasuryPayment => "53",
            Payment::CreditCard => "54",
            Payment::DebitCard => "55",
            Payment::Bankgiro => "56",
            Payment::StandingAgreement => "57",
            Payment::SepaCreditTransfer => "58",
            Payment::SepaDirectDebit => "59",
            Payment::PromissoryNote => "60",
            Payment::PromissoryNoteSignedByDebtor => "61",
            Payment::PromissoryNoteSignedByDebtorAndEndorsedByABank => "62",
            Payment::PromissoryNoteSignedByDebtorAndEndorsedByA => "63",
            Payment::PromissoryNoteSignedByABank => "64",
            Payment::PromissoryNoteSignedByABankAndEndorsedByAnother => "65",
            Payment::PromissoryNoteSignedByAThirdParty => "66",
            Payment::PromissoryNoteSignedByAThirdPartyAndEndorsedByA => "67",
            Payment::OnlinePaymentService => "68",
            Payment::TransferAdvice => "69",
            Payment::BillDrawnByCreditorOnDebtor => "70",
            Payment::BillDrawnByCreditorOnABank => "74",
            Payment::BillDrawnByCreditorEndorsedByAnotherBank => "75",
            Payment::BillDrawnByCreditorOnABankAndEndorsedByA => "76",
            Payment::BillDrawnByCreditorOnAThirdParty => "77",
            Payment::BillDrawnByCreditorOnThirdPartyAcceptedAnd => "78",
            Payment::NotTransferableBankersDraft => "91",
            Payment::NotTransferableLocalCheque => "92",
            Payment::ReferenceGiro => "93",
            Payment::UrgentGiro => "94",
            Payment::FreeFormatGiro => "95",
            Payment::RequestedMethodForPaymentWasNotUsed => "96",
            Payment::ClearingBetweenPartners => "97",
            Payment::JpElectronicallyRecordedMonetaryClaims => "98",
            Payment::MutuallyDefined => "ZZZ",
        }
    }
}

impl crate::Description for Payment {
    fn description(&self) -> &str {
        match self {
            Payment::InstrumentNotDefined => "Instrument not defined",
            Payment::AutomatedClearingHouseCredit => "Automated clearing house credit",
            Payment::AutomatedClearingHouseDebit => "Automated clearing house debit",
            Payment::AchDemandDebitReversal => "ACH demand debit reversal",
            Payment::AchDemandCreditReversal => "ACH demand credit reversal",
            Payment::AchDemandCredit => "ACH demand credit",
            Payment::AchDemandDebit => "ACH demand debit",
            Payment::Hold => "Hold",
            Payment::NationalOrRegionalClearing => "National or regional clearing",
            Payment::InCash => "In cash",
            Payment::AchSavingsCreditReversal => "ACH savings credit reversal",
            Payment::AchSavingsDebitReversal => "ACH savings debit reversal",
            Payment::AchSavingsCredit => "ACH savings credit",
            Payment::AchSavingsDebit => "ACH savings debit",
            Payment::BookentryCredit => "Bookentry credit",
            Payment::BookentryDebit => "Bookentry debit",
            Payment::AchDemandCashConcentrationDisbursementCcdCredit => {
                "ACH demand cash concentration/disbursement (CCD) credit"
            }
            Payment::AchDemandCashConcentrationDisbursementCcdDebit => {
                "ACH demand cash concentration/disbursement (CCD) debit"
            }
            Payment::AchDemandCorporateTradePaymentCtpCredit => {
                "ACH demand corporate trade payment (CTP) credit"
            }
            Payment::Cheque => "Cheque",
            Payment::BankersDraft => "Banker's draft",
            Payment::CertifiedBankersDraft => "Certified banker's draft",
            Payment::BankChequeIssuedByABankingOrSimilarEstablishment => {
                "Bank cheque (issued by a banking or similar establishment)"
            }
            Payment::BillExchangeAwaitingAcceptance => "Bill of exchange awaiting acceptance",
            Payment::CertifiedCheque => "Certified cheque",
            Payment::LocalCheque => "Local cheque",
            Payment::AchDemandCorporateTradePaymentCtpDebit => {
                "ACH demand corporate trade payment (CTP) debit"
            }
            Payment::AchDemandCorporateTradeExchangeCtxCredit => {
                "ACH demand corporate trade exchange (CTX) credit"
            }
            Payment::AchDemandCorporateTradeExchangeCtxDebit => {
                "ACH demand corporate trade exchange (CTX) debit"
            }
            Payment::CreditTransfer => "Credit transfer",
            Payment::DebitTransfer => "Debit transfer",
            Payment::AchDemandCashConcentrationDisbursementPlusCcd => {
                "ACH demand cash concentration/disbursement plus (CCD+)"
            }
            Payment::AchDemandCashConcentrationDisbursementPlusCcd_Dup => {
                "ACH demand cash concentration/disbursement plus (CCD+)"
            }
            Payment::AchPrearrangedPaymentAndDepositPpd => {
                "ACH prearranged payment and deposit (PPD)"
            }
            Payment::AchSavingsCashConcentrationDisbursementCcdCredit => {
                "ACH savings cash concentration/disbursement (CCD) credit"
            }
            Payment::AchSavingsCashConcentrationDisbursementCcdDebit => {
                "ACH savings cash concentration/disbursement (CCD) debit"
            }
            Payment::AchSavingsCorporateTradePaymentCtpCredit => {
                "ACH savings corporate trade payment (CTP) credit"
            }
            Payment::AchSavingsCorporateTradePaymentCtpDebit => {
                "ACH savings corporate trade payment (CTP) debit"
            }
            Payment::AchSavingsCorporateTradeExchangeCtxCredit => {
                "ACH savings corporate trade exchange (CTX) credit"
            }
            Payment::AchSavingsCorporateTradeExchangeCtxDebit => {
                "ACH savings corporate trade exchange (CTX) debit"
            }
            Payment::AchSavingsCashConcentrationDisbursementPlusCcd => {
                "ACH savings cash concentration/disbursement plus (CCD+)"
            }
            Payment::PaymentToBankAccount => "Payment to bank account",
            Payment::AchSavingsCashConcentrationDisbursementPlusCcd_Dup => {
                "ACH savings cash concentration/disbursement plus (CCD+)"
            }
            Payment::AcceptedBillExchange => "Accepted bill of exchange",
            Payment::ReferencedHomeBankingCreditTransfer => {
                "Referenced home-banking credit transfer"
            }
            Payment::InterbankDebitTransfer => "Interbank debit transfer",
            Payment::HomeBankingDebitTransfer => "Home-banking debit transfer",
            Payment::BankCard => "Bank card",
            Payment::DirectDebit => "Direct debit",
            Payment::PaymentByPostgiro => "Payment by postgiro",
            Payment::FrNorme697TelereglementCfonbFrenchOrganisationFor => {
                "FR, norme 6 97-Telereglement CFONB (French Organisation for"
            }
            Payment::UrgentCommercialPayment => "Urgent commercial payment",
            Payment::UrgentTreasuryPayment => "Urgent Treasury Payment",
            Payment::CreditCard => "Credit card",
            Payment::DebitCard => "Debit card",
            Payment::Bankgiro => "Bankgiro",
            Payment::StandingAgreement => "Standing agreement",
            Payment::SepaCreditTransfer => "SEPA credit transfer",
            Payment::SepaDirectDebit => "SEPA direct debit",
            Payment::PromissoryNote => "Promissory note",
            Payment::PromissoryNoteSignedByDebtor => "Promissory note signed by the debtor",
            Payment::PromissoryNoteSignedByDebtorAndEndorsedByABank => {
                "Promissory note signed by the debtor and endorsed by a bank"
            }
            Payment::PromissoryNoteSignedByDebtorAndEndorsedByA => {
                "Promissory note signed by the debtor and endorsed by a"
            }
            Payment::PromissoryNoteSignedByABank => "Promissory note signed by a bank",
            Payment::PromissoryNoteSignedByABankAndEndorsedByAnother => {
                "Promissory note signed by a bank and endorsed by another"
            }
            Payment::PromissoryNoteSignedByAThirdParty => "Promissory note signed by a third party",
            Payment::PromissoryNoteSignedByAThirdPartyAndEndorsedByA => {
                "Promissory note signed by a third party and endorsed by a"
            }
            Payment::OnlinePaymentService => "Online payment service",
            Payment::TransferAdvice => "Transfer Advice",
            Payment::BillDrawnByCreditorOnDebtor => "Bill drawn by the creditor on the debtor",
            Payment::BillDrawnByCreditorOnABank => "Bill drawn by the creditor on a bank",
            Payment::BillDrawnByCreditorEndorsedByAnotherBank => {
                "Bill drawn by the creditor, endorsed by another bank"
            }
            Payment::BillDrawnByCreditorOnABankAndEndorsedByA => {
                "Bill drawn by the creditor on a bank and endorsed by a"
            }
            Payment::BillDrawnByCreditorOnAThirdParty => {
                "Bill drawn by the creditor on a third party"
            }
            Payment::BillDrawnByCreditorOnThirdPartyAcceptedAnd => {
                "Bill drawn by creditor on third party, accepted and"
            }
            Payment::NotTransferableBankersDraft => "Not transferable banker's draft",
            Payment::NotTransferableLocalCheque => "Not transferable local cheque",
            Payment::ReferenceGiro => "Reference giro",
            Payment::UrgentGiro => "Urgent giro",
            Payment::FreeFormatGiro => "Free format giro",
            Payment::RequestedMethodForPaymentWasNotUsed => {
                "Requested method for payment was not used"
            }
            Payment::ClearingBetweenPartners => "Clearing between partners",
            Payment::JpElectronicallyRecordedMonetaryClaims => {
                "JP, Electronically Recorded Monetary Claims"
            }
            Payment::MutuallyDefined => "Mutually defined",
        }
    }
}

impl crate::FromCode for Payment {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "1" => Some(Payment::InstrumentNotDefined),
            "2" => Some(Payment::AutomatedClearingHouseCredit),
            "3" => Some(Payment::AutomatedClearingHouseDebit),
            "4" => Some(Payment::AchDemandDebitReversal),
            "5" => Some(Payment::AchDemandCreditReversal),
            "6" => Some(Payment::AchDemandCredit),
            "7" => Some(Payment::AchDemandDebit),
            "8" => Some(Payment::Hold),
            "9" => Some(Payment::NationalOrRegionalClearing),
            "10" => Some(Payment::InCash),
            "11" => Some(Payment::AchSavingsCreditReversal),
            "12" => Some(Payment::AchSavingsDebitReversal),
            "13" => Some(Payment::AchSavingsCredit),
            "14" => Some(Payment::AchSavingsDebit),
            "15" => Some(Payment::BookentryCredit),
            "16" => Some(Payment::BookentryDebit),
            "17" => Some(Payment::AchDemandCashConcentrationDisbursementCcdCredit),
            "18" => Some(Payment::AchDemandCashConcentrationDisbursementCcdDebit),
            "19" => Some(Payment::AchDemandCorporateTradePaymentCtpCredit),
            "20" => Some(Payment::Cheque),
            "21" => Some(Payment::BankersDraft),
            "22" => Some(Payment::CertifiedBankersDraft),
            "23" => Some(Payment::BankChequeIssuedByABankingOrSimilarEstablishment),
            "24" => Some(Payment::BillExchangeAwaitingAcceptance),
            "25" => Some(Payment::CertifiedCheque),
            "26" => Some(Payment::LocalCheque),
            "27" => Some(Payment::AchDemandCorporateTradePaymentCtpDebit),
            "28" => Some(Payment::AchDemandCorporateTradeExchangeCtxCredit),
            "29" => Some(Payment::AchDemandCorporateTradeExchangeCtxDebit),
            "30" => Some(Payment::CreditTransfer),
            "31" => Some(Payment::DebitTransfer),
            "32" => Some(Payment::AchDemandCashConcentrationDisbursementPlusCcd),
            "33" => Some(Payment::AchDemandCashConcentrationDisbursementPlusCcd_Dup),
            "34" => Some(Payment::AchPrearrangedPaymentAndDepositPpd),
            "35" => Some(Payment::AchSavingsCashConcentrationDisbursementCcdCredit),
            "36" => Some(Payment::AchSavingsCashConcentrationDisbursementCcdDebit),
            "37" => Some(Payment::AchSavingsCorporateTradePaymentCtpCredit),
            "38" => Some(Payment::AchSavingsCorporateTradePaymentCtpDebit),
            "39" => Some(Payment::AchSavingsCorporateTradeExchangeCtxCredit),
            "40" => Some(Payment::AchSavingsCorporateTradeExchangeCtxDebit),
            "41" => Some(Payment::AchSavingsCashConcentrationDisbursementPlusCcd),
            "42" => Some(Payment::PaymentToBankAccount),
            "43" => Some(Payment::AchSavingsCashConcentrationDisbursementPlusCcd_Dup),
            "44" => Some(Payment::AcceptedBillExchange),
            "45" => Some(Payment::ReferencedHomeBankingCreditTransfer),
            "46" => Some(Payment::InterbankDebitTransfer),
            "47" => Some(Payment::HomeBankingDebitTransfer),
            "48" => Some(Payment::BankCard),
            "49" => Some(Payment::DirectDebit),
            "50" => Some(Payment::PaymentByPostgiro),
            "51" => Some(Payment::FrNorme697TelereglementCfonbFrenchOrganisationFor),
            "52" => Some(Payment::UrgentCommercialPayment),
            "53" => Some(Payment::UrgentTreasuryPayment),
            "54" => Some(Payment::CreditCard),
            "55" => Some(Payment::DebitCard),
            "56" => Some(Payment::Bankgiro),
            "57" => Some(Payment::StandingAgreement),
            "58" => Some(Payment::SepaCreditTransfer),
            "59" => Some(Payment::SepaDirectDebit),
            "60" => Some(Payment::PromissoryNote),
            "61" => Some(Payment::PromissoryNoteSignedByDebtor),
            "62" => Some(Payment::PromissoryNoteSignedByDebtorAndEndorsedByABank),
            "63" => Some(Payment::PromissoryNoteSignedByDebtorAndEndorsedByA),
            "64" => Some(Payment::PromissoryNoteSignedByABank),
            "65" => Some(Payment::PromissoryNoteSignedByABankAndEndorsedByAnother),
            "66" => Some(Payment::PromissoryNoteSignedByAThirdParty),
            "67" => Some(Payment::PromissoryNoteSignedByAThirdPartyAndEndorsedByA),
            "68" => Some(Payment::OnlinePaymentService),
            "69" => Some(Payment::TransferAdvice),
            "70" => Some(Payment::BillDrawnByCreditorOnDebtor),
            "74" => Some(Payment::BillDrawnByCreditorOnABank),
            "75" => Some(Payment::BillDrawnByCreditorEndorsedByAnotherBank),
            "76" => Some(Payment::BillDrawnByCreditorOnABankAndEndorsedByA),
            "77" => Some(Payment::BillDrawnByCreditorOnAThirdParty),
            "78" => Some(Payment::BillDrawnByCreditorOnThirdPartyAcceptedAnd),
            "91" => Some(Payment::NotTransferableBankersDraft),
            "92" => Some(Payment::NotTransferableLocalCheque),
            "93" => Some(Payment::ReferenceGiro),
            "94" => Some(Payment::UrgentGiro),
            "95" => Some(Payment::FreeFormatGiro),
            "96" => Some(Payment::RequestedMethodForPaymentWasNotUsed),
            "97" => Some(Payment::ClearingBetweenPartners),
            "98" => Some(Payment::JpElectronicallyRecordedMonetaryClaims),
            "ZZZ" => Some(Payment::MutuallyDefined),
            _ => None,
        }
    }
}
