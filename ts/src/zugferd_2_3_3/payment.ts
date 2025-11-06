export enum Payment {
  /**
   * Instrument not defined
   */
  InstrumentNotDefined = "1",
  /**
   * Automated clearing house credit
   */
  AutomatedClearingHouseCredit = "2",
  /**
   * Automated clearing house debit
   */
  AutomatedClearingHouseDebit = "3",
  /**
   * ACH demand debit reversal
   */
  AchDemandDebitReversal = "4",
  /**
   * ACH demand credit reversal
   */
  AchDemandCreditReversal = "5",
  /**
   * ACH demand credit
   */
  AchDemandCredit = "6",
  /**
   * ACH demand debit
   */
  AchDemandDebit = "7",
  /**
   * Hold
   */
  Hold = "8",
  /**
   * National or regional clearing
   */
  NationalOrRegionalClearing = "9",
  /**
   * In cash
   */
  InCash = "10",
  /**
   * ACH savings credit reversal
   */
  AchSavingsCreditReversal = "11",
  /**
   * ACH savings debit reversal
   */
  AchSavingsDebitReversal = "12",
  /**
   * ACH savings credit
   */
  AchSavingsCredit = "13",
  /**
   * ACH savings debit
   */
  AchSavingsDebit = "14",
  /**
   * Bookentry credit
   */
  BookentryCredit = "15",
  /**
   * Bookentry debit
   */
  BookentryDebit = "16",
  /**
   * ACH demand cash concentration/disbursement (CCD) credit
   */
  AchDemandCashConcentrationDisbursementCcdCredit = "17",
  /**
   * ACH demand cash concentration/disbursement (CCD) debit
   */
  AchDemandCashConcentrationDisbursementCcdDebit = "18",
  /**
   * ACH demand corporate trade payment (CTP) credit
   */
  AchDemandCorporateTradePaymentCtpCredit = "19",
  /**
   * Cheque
   */
  Cheque = "20",
  /**
   * Banker's draft
   */
  BankersDraft = "21",
  /**
   * Certified banker's draft
   */
  CertifiedBankersDraft = "22",
  /**
   * Bank cheque (issued by a banking or similar establishment)
   */
  BankChequeIssuedByABankingOrSimilarEstablishment = "23",
  /**
   * Bill of exchange awaiting acceptance
   */
  BillExchangeAwaitingAcceptance = "24",
  /**
   * Certified cheque
   */
  CertifiedCheque = "25",
  /**
   * Local cheque
   */
  LocalCheque = "26",
  /**
   * ACH demand corporate trade payment (CTP) debit
   */
  AchDemandCorporateTradePaymentCtpDebit = "27",
  /**
   * ACH demand corporate trade exchange (CTX) credit
   */
  AchDemandCorporateTradeExchangeCtxCredit = "28",
  /**
   * ACH demand corporate trade exchange (CTX) debit
   */
  AchDemandCorporateTradeExchangeCtxDebit = "29",
  /**
   * Credit transfer
   *
   * non-SEPA
   */
  CreditTransfer = "30",
  /**
   * Debit transfer
   *
   * non-SEPA
   */
  DebitTransfer = "31",
  /**
   * ACH demand cash concentration/disbursement plus (CCD+)
   */
  AchDemandCashConcentrationDisbursementPlusCcd = "32",
  /**
   * ACH demand cash concentration/disbursement plus (CCD+)
   */
  AchDemandCashConcentrationDisbursementPlusCcd_Dup = "33",
  /**
   * ACH prearranged payment and deposit (PPD)
   */
  AchPrearrangedPaymentAndDepositPpd = "34",
  /**
   * ACH savings cash concentration/disbursement (CCD) credit
   */
  AchSavingsCashConcentrationDisbursementCcdCredit = "35",
  /**
   * ACH savings cash concentration/disbursement (CCD) debit
   */
  AchSavingsCashConcentrationDisbursementCcdDebit = "36",
  /**
   * ACH savings corporate trade payment (CTP) credit
   */
  AchSavingsCorporateTradePaymentCtpCredit = "37",
  /**
   * ACH savings corporate trade payment (CTP) debit
   */
  AchSavingsCorporateTradePaymentCtpDebit = "38",
  /**
   * ACH savings corporate trade exchange (CTX) credit
   */
  AchSavingsCorporateTradeExchangeCtxCredit = "39",
  /**
   * ACH savings corporate trade exchange (CTX) debit
   */
  AchSavingsCorporateTradeExchangeCtxDebit = "40",
  /**
   * ACH savings cash concentration/disbursement plus (CCD+)
   */
  AchSavingsCashConcentrationDisbursementPlusCcd = "41",
  /**
   * Payment to bank account
   */
  PaymentToBankAccount = "42",
  /**
   * ACH savings cash concentration/disbursement plus (CCD+)
   */
  AchSavingsCashConcentrationDisbursementPlusCcd_Dup = "43",
  /**
   * Accepted bill of exchange
   */
  AcceptedBillExchange = "44",
  /**
   * Referenced home-banking credit transfer
   */
  ReferencedHomeBankingCreditTransfer = "45",
  /**
   * Interbank debit transfer
   */
  InterbankDebitTransfer = "46",
  /**
   * Home-banking debit transfer
   */
  HomeBankingDebitTransfer = "47",
  /**
   * Bank card
   *
   * Use for all payment cards
   */
  BankCard = "48",
  /**
   * Direct debit
   */
  DirectDebit = "49",
  /**
   * Payment by postgiro
   */
  PaymentByPostgiro = "50",
  /**
   * FR, norme 6 97-Telereglement CFONB (French Organisation for
   */
  FrNorme697TelereglementCfonbFrenchOrganisationFor = "51",
  /**
   * Urgent commercial payment
   */
  UrgentCommercialPayment = "52",
  /**
   * Urgent Treasury Payment
   */
  UrgentTreasuryPayment = "53",
  /**
   * Credit card
   */
  CreditCard = "54",
  /**
   * Debit card
   */
  DebitCard = "55",
  /**
   * Bankgiro
   */
  Bankgiro = "56",
  /**
   * Standing agreement
   *
   * Contractual payment means
   */
  StandingAgreement = "57",
  /**
   * SEPA credit transfer
   *
   * SEPA
   */
  SepaCreditTransfer = "58",
  /**
   * SEPA direct debit
   *
   * SEPA
   */
  SepaDirectDebit = "59",
  /**
   * Promissory note
   */
  PromissoryNote = "60",
  /**
   * Promissory note signed by the debtor
   */
  PromissoryNoteSignedByDebtor = "61",
  /**
   * Promissory note signed by the debtor and endorsed by a bank
   */
  PromissoryNoteSignedByDebtorAndEndorsedByABank = "62",
  /**
   * Promissory note signed by the debtor and endorsed by a
   */
  PromissoryNoteSignedByDebtorAndEndorsedByA = "63",
  /**
   * Promissory note signed by a bank
   */
  PromissoryNoteSignedByABank = "64",
  /**
   * Promissory note signed by a bank and endorsed by another
   */
  PromissoryNoteSignedByABankAndEndorsedByAnother = "65",
  /**
   * Promissory note signed by a third party
   */
  PromissoryNoteSignedByAThirdParty = "66",
  /**
   * Promissory note signed by a third party and endorsed by a
   */
  PromissoryNoteSignedByAThirdPartyAndEndorsedByA = "67",
  /**
   * Online payment service
   */
  OnlinePaymentService = "68",
  /**
   * Transfer Advice
   */
  TransferAdvice = "69",
  /**
   * Bill drawn by the creditor on the debtor
   */
  BillDrawnByCreditorOnDebtor = "70",
  /**
   * Bill drawn by the creditor on a bank
   */
  BillDrawnByCreditorOnABank = "74",
  /**
   * Bill drawn by the creditor, endorsed by another bank
   */
  BillDrawnByCreditorEndorsedByAnotherBank = "75",
  /**
   * Bill drawn by the creditor on a bank and endorsed by a
   */
  BillDrawnByCreditorOnABankAndEndorsedByA = "76",
  /**
   * Bill drawn by the creditor on a third party
   */
  BillDrawnByCreditorOnAThirdParty = "77",
  /**
   * Bill drawn by creditor on third party, accepted and
   */
  BillDrawnByCreditorOnThirdPartyAcceptedAnd = "78",
  /**
   * Not transferable banker's draft
   */
  NotTransferableBankersDraft = "91",
  /**
   * Not transferable local cheque
   */
  NotTransferableLocalCheque = "92",
  /**
   * Reference giro
   */
  ReferenceGiro = "93",
  /**
   * Urgent giro
   */
  UrgentGiro = "94",
  /**
   * Free format giro
   */
  FreeFormatGiro = "95",
  /**
   * Requested method for payment was not used
   */
  RequestedMethodForPaymentWasNotUsed = "96",
  /**
   * Clearing between partners
   */
  ClearingBetweenPartners = "97",
  /**
   * JP, Electronically Recorded Monetary Claims
   */
  JpElectronicallyRecordedMonetaryClaims = "98",
  /**
   * Mutually defined
   */
  MutuallyDefined = "ZZZ",
}

export function description(value: Payment): string {
  switch (value) {
    case Payment.InstrumentNotDefined:
      return "Instrument not defined";
    case Payment.AutomatedClearingHouseCredit:
      return "Automated clearing house credit";
    case Payment.AutomatedClearingHouseDebit:
      return "Automated clearing house debit";
    case Payment.AchDemandDebitReversal:
      return "ACH demand debit reversal";
    case Payment.AchDemandCreditReversal:
      return "ACH demand credit reversal";
    case Payment.AchDemandCredit:
      return "ACH demand credit";
    case Payment.AchDemandDebit:
      return "ACH demand debit";
    case Payment.Hold:
      return "Hold";
    case Payment.NationalOrRegionalClearing:
      return "National or regional clearing";
    case Payment.InCash:
      return "In cash";
    case Payment.AchSavingsCreditReversal:
      return "ACH savings credit reversal";
    case Payment.AchSavingsDebitReversal:
      return "ACH savings debit reversal";
    case Payment.AchSavingsCredit:
      return "ACH savings credit";
    case Payment.AchSavingsDebit:
      return "ACH savings debit";
    case Payment.BookentryCredit:
      return "Bookentry credit";
    case Payment.BookentryDebit:
      return "Bookentry debit";
    case Payment.AchDemandCashConcentrationDisbursementCcdCredit:
      return "ACH demand cash concentration/disbursement (CCD) credit";
    case Payment.AchDemandCashConcentrationDisbursementCcdDebit:
      return "ACH demand cash concentration/disbursement (CCD) debit";
    case Payment.AchDemandCorporateTradePaymentCtpCredit:
      return "ACH demand corporate trade payment (CTP) credit";
    case Payment.Cheque:
      return "Cheque";
    case Payment.BankersDraft:
      return "Banker's draft";
    case Payment.CertifiedBankersDraft:
      return "Certified banker's draft";
    case Payment.BankChequeIssuedByABankingOrSimilarEstablishment:
      return "Bank cheque (issued by a banking or similar establishment)";
    case Payment.BillExchangeAwaitingAcceptance:
      return "Bill of exchange awaiting acceptance";
    case Payment.CertifiedCheque:
      return "Certified cheque";
    case Payment.LocalCheque:
      return "Local cheque";
    case Payment.AchDemandCorporateTradePaymentCtpDebit:
      return "ACH demand corporate trade payment (CTP) debit";
    case Payment.AchDemandCorporateTradeExchangeCtxCredit:
      return "ACH demand corporate trade exchange (CTX) credit";
    case Payment.AchDemandCorporateTradeExchangeCtxDebit:
      return "ACH demand corporate trade exchange (CTX) debit";
    case Payment.CreditTransfer:
      return "Credit transfer";
    case Payment.DebitTransfer:
      return "Debit transfer";
    case Payment.AchDemandCashConcentrationDisbursementPlusCcd:
      return "ACH demand cash concentration/disbursement plus (CCD+)";
    case Payment.AchDemandCashConcentrationDisbursementPlusCcd_Dup:
      return "ACH demand cash concentration/disbursement plus (CCD+)";
    case Payment.AchPrearrangedPaymentAndDepositPpd:
      return "ACH prearranged payment and deposit (PPD)";
    case Payment.AchSavingsCashConcentrationDisbursementCcdCredit:
      return "ACH savings cash concentration/disbursement (CCD) credit";
    case Payment.AchSavingsCashConcentrationDisbursementCcdDebit:
      return "ACH savings cash concentration/disbursement (CCD) debit";
    case Payment.AchSavingsCorporateTradePaymentCtpCredit:
      return "ACH savings corporate trade payment (CTP) credit";
    case Payment.AchSavingsCorporateTradePaymentCtpDebit:
      return "ACH savings corporate trade payment (CTP) debit";
    case Payment.AchSavingsCorporateTradeExchangeCtxCredit:
      return "ACH savings corporate trade exchange (CTX) credit";
    case Payment.AchSavingsCorporateTradeExchangeCtxDebit:
      return "ACH savings corporate trade exchange (CTX) debit";
    case Payment.AchSavingsCashConcentrationDisbursementPlusCcd:
      return "ACH savings cash concentration/disbursement plus (CCD+)";
    case Payment.PaymentToBankAccount:
      return "Payment to bank account";
    case Payment.AchSavingsCashConcentrationDisbursementPlusCcd_Dup:
      return "ACH savings cash concentration/disbursement plus (CCD+)";
    case Payment.AcceptedBillExchange:
      return "Accepted bill of exchange";
    case Payment.ReferencedHomeBankingCreditTransfer:
      return "Referenced home-banking credit transfer";
    case Payment.InterbankDebitTransfer:
      return "Interbank debit transfer";
    case Payment.HomeBankingDebitTransfer:
      return "Home-banking debit transfer";
    case Payment.BankCard:
      return "Bank card";
    case Payment.DirectDebit:
      return "Direct debit";
    case Payment.PaymentByPostgiro:
      return "Payment by postgiro";
    case Payment.FrNorme697TelereglementCfonbFrenchOrganisationFor:
      return "FR, norme 6 97-Telereglement CFONB (French Organisation for";
    case Payment.UrgentCommercialPayment:
      return "Urgent commercial payment";
    case Payment.UrgentTreasuryPayment:
      return "Urgent Treasury Payment";
    case Payment.CreditCard:
      return "Credit card";
    case Payment.DebitCard:
      return "Debit card";
    case Payment.Bankgiro:
      return "Bankgiro";
    case Payment.StandingAgreement:
      return "Standing agreement";
    case Payment.SepaCreditTransfer:
      return "SEPA credit transfer";
    case Payment.SepaDirectDebit:
      return "SEPA direct debit";
    case Payment.PromissoryNote:
      return "Promissory note";
    case Payment.PromissoryNoteSignedByDebtor:
      return "Promissory note signed by the debtor";
    case Payment.PromissoryNoteSignedByDebtorAndEndorsedByABank:
      return "Promissory note signed by the debtor and endorsed by a bank";
    case Payment.PromissoryNoteSignedByDebtorAndEndorsedByA:
      return "Promissory note signed by the debtor and endorsed by a";
    case Payment.PromissoryNoteSignedByABank:
      return "Promissory note signed by a bank";
    case Payment.PromissoryNoteSignedByABankAndEndorsedByAnother:
      return "Promissory note signed by a bank and endorsed by another";
    case Payment.PromissoryNoteSignedByAThirdParty:
      return "Promissory note signed by a third party";
    case Payment.PromissoryNoteSignedByAThirdPartyAndEndorsedByA:
      return "Promissory note signed by a third party and endorsed by a";
    case Payment.OnlinePaymentService:
      return "Online payment service";
    case Payment.TransferAdvice:
      return "Transfer Advice";
    case Payment.BillDrawnByCreditorOnDebtor:
      return "Bill drawn by the creditor on the debtor";
    case Payment.BillDrawnByCreditorOnABank:
      return "Bill drawn by the creditor on a bank";
    case Payment.BillDrawnByCreditorEndorsedByAnotherBank:
      return "Bill drawn by the creditor, endorsed by another bank";
    case Payment.BillDrawnByCreditorOnABankAndEndorsedByA:
      return "Bill drawn by the creditor on a bank and endorsed by a";
    case Payment.BillDrawnByCreditorOnAThirdParty:
      return "Bill drawn by the creditor on a third party";
    case Payment.BillDrawnByCreditorOnThirdPartyAcceptedAnd:
      return "Bill drawn by creditor on third party, accepted and";
    case Payment.NotTransferableBankersDraft:
      return "Not transferable banker's draft";
    case Payment.NotTransferableLocalCheque:
      return "Not transferable local cheque";
    case Payment.ReferenceGiro:
      return "Reference giro";
    case Payment.UrgentGiro:
      return "Urgent giro";
    case Payment.FreeFormatGiro:
      return "Free format giro";
    case Payment.RequestedMethodForPaymentWasNotUsed:
      return "Requested method for payment was not used";
    case Payment.ClearingBetweenPartners:
      return "Clearing between partners";
    case Payment.JpElectronicallyRecordedMonetaryClaims:
      return "JP, Electronically Recorded Monetary Claims";
    case Payment.MutuallyDefined:
      return "Mutually defined";
  }
}
