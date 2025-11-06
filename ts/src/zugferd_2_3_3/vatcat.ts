export enum VATCAT {
  /**
   * Petroleum tax
   *
   * A tax levied on the volume of petroleum being transacted.
   */
  PetroleumTax = "AAA",
  /**
   * Provisional countervailing duty cash
   *
   * Countervailing duty paid in cash prior to a formal finding of subsidization by Customs.
   */
  ProvisionalCountervailingDutyCash = "AAB",
  /**
   * Provisional countervailing duty bond
   *
   * Countervailing duty paid by posting a bond during an investigation period prior to a formal decision on subsidization by Customs.
   */
  ProvisionalCountervailingDutyBond = "AAC",
  /**
   * Tobacco tax
   *
   * A tax levied on tobacco products.
   */
  TobaccoTax = "AAD",
  /**
   * Energy fee
   *
   * General fee or tax for the use of energy.
   */
  EnergyFee = "AAE",
  /**
   * Coffee tax
   *
   * A tax levied specifically on coffee products.
   */
  CoffeeTax = "AAF",
  /**
   * Harmonised sales tax, Canadian
   *
   * A harmonized sales tax consisting of a goods and service tax, a Canadian provincial sales tax and, as applicable, a Quebec sales tax which is recoverable.
   */
  HarmonisedSalesTaxCanadian = "AAG",
  /**
   * Quebec sales tax
   *
   * A sales tax charged within the Canadian province of Quebec which is recoverable.
   */
  QuebecSalesTax = "AAH",
  /**
   * Canadian provincial sales tax
   *
   * A sales tax charged within Canadian provinces which is non-recoverable.
   */
  CanadianProvincialSalesTax = "AAI",
  /**
   * Tax on replacement part
   *
   * A tax levied on a replacement part, where the original part is returned.
   */
  TaxOnReplacementPart = "AAJ",
  /**
   * Mineral oil tax
   *
   * Tax that is levied specifically on products containing mineral oil.
   */
  MineralOilTax = "AAK",
  /**
   * Special tax
   *
   * To indicate a special type of tax.
   */
  SpecialTax = "AAL",
  /**
   * Insurance tax
   *
   * A tax levied specifically on insurances.
   */
  InsuranceTax = "AAM",
  /**
   * Provincial Cannabis Tax
   *
   * A tax levied on Cannabis products
   */
  ProvincialCannabisTax = "AAO",
  /**
   * Outstanding duty interest
   *
   * Fee levied due to outstanding duties to be paid
   */
  OutstandingDutyInterest = "AAP",
  /**
   * Anti-dumping duty
   *
   * Duty applied to goods ruled to have been dumped in an import market at a price lower than that in the exporter's domestic market.
   */
  AntiDumpingDuty = "ADD",
  /**
   * Stamp duty (Imposta di Bollo)
   *
   * Tax required in Italy, which may be fixed or graduated in various circumstances (e.g. VAT exempt documents or bank receipts).
   */
  StampDutyImpostaDiBollo = "BOL",
  /**
   * Agricultural levy
   *
   * Levy imposed on agricultural products where there is a difference between the selling price between trading countries.
   */
  AgriculturalLevy = "CAP",
  /**
   * Car tax
   *
   * A tax that is levied on the value of the automobile.
   */
  CarTax = "CAR",
  /**
   * Paper consortium tax (Italy)
   *
   * Italian Paper consortium tax.
   */
  PaperConsortiumTaxItaly = "COC",
  /**
   * Commodity specific tax
   *
   * Tax related to a specified commodity, e.g. illuminants, salts.
   */
  CommoditySpecificTax = "CST",
  /**
   * Customs duty
   *
   * Duties laid down in the Customs tariff, to which goods are liable on entering or leaving the Customs territory (CCC).
   */
  CustomsDuty = "CUD",
  /**
   * Countervailing duty
   *
   * A duty on imported goods applied for compensate for subsidies granted to those goods in the exporting country.
   */
  CountervailingDuty = "CVD",
  /**
   * Environmental tax
   *
   * Tax assessed for funding or assuring environmental protection or clean-up.
   */
  EnvironmentalTax = "ENV",
  /**
   * Excise duty
   *
   * Customs or fiscal authorities code to identify a specific or ad valorem levy on a specific commodity, applied either domestically or at time of importation.
   */
  ExciseDuty = "EXC",
  /**
   * Agricultural export rebate
   *
   * Monetary rebate given to the seller in certain circumstances when agricultural products are exported.
   */
  AgriculturalExportRebate = "EXP",
  /**
   * Federal excise tax
   *
   * Tax levied by the federal government on the manufacture of specific items.
   */
  FederalExciseTax = "FET",
  /**
   * Free
   *
   * No tax levied.
   */
  Free = "FRE",
  /**
   * General construction tax
   *
   * General tax for construction.
   */
  GeneralConstructionTax = "GCN",
  /**
   * Goods and services tax
   *
   * Tax levied on the final consumption of goods and services throughout the production and distribution chain.
   */
  GoodsAndServicesTax = "GST",
  /**
   * Illuminants tax
   *
   * Tax of illuminants.
   */
  IlluminantsTax = "ILL",
  /**
   * Import tax
   *
   * Tax assessed on imports.
   */
  ImportTax = "IMP",
  /**
   * Individual tax
   *
   * A tax levied based on an individual's ability to pay.
   */
  IndividualTax = "IND",
  /**
   * Business license fee
   *
   * Government assessed charge for permit to do business.
   */
  BusinessLicenseFee = "LAC",
  /**
   * Local construction tax
   *
   * Local tax for construction.
   */
  LocalConstructionTax = "LCN",
  /**
   * Light dues payable
   *
   * Fee levied on a vessel to pay for port navigation lights.
   */
  LightDuesPayable = "LDP",
  /**
   * Local sales tax
   *
   * Assessment charges on sale of goods or services by city, borough country or other taxing authorities below state or provincial level.
   */
  LocalSalesTax = "LOC",
  /**
   * Lust tax
   *
   * Tax imposed for clean-up of leaky underground storage tanks.
   */
  LustTax = "LST",
  /**
   * Monetary compensatory amount
   *
   * Levy on Common Agricultural Policy (European Union) goods used to compensate for fluctuating currencies between member states.
   */
  MonetaryCompensatoryAmount = "MCA",
  /**
   * Miscellaneous cash deposit
   *
   * Duty paid and held on deposit, by Customs, during an investigation period prior to a final decision being made on any aspect related to imported goods (except valuation) by Customs.
   */
  MiscellaneousCashDeposit = "MCD",
  /**
   * Other taxes
   *
   * Unspecified, miscellaneous tax charges.
   */
  OtherTaxes = "OTH",
  /**
   * Provisional duty bond
   *
   * Anti-dumping duty paid by posting a bond during an investigation period prior to a formal decision on dumping by Customs.
   */
  ProvisionalDutyBond = "PDB",
  /**
   * Provisional duty cash
   *
   * Anti-dumping duty paid in cash prior to a formal finding of dumping by Customs.
   */
  ProvisionalDutyCash = "PDC",
  /**
   * Preference duty
   *
   * Duties laid down in the Customs tariff, to which goods are liable on entering or leaving the Customs territory falling under a preferential regime such as Generalised System of Preferences (GSP).
   */
  PreferenceDuty = "PRF",
  /**
   * Special construction tax
   *
   * Special tax for construction.
   */
  SpecialConstructionTax = "SCN",
  /**
   * Shifted social securities
   *
   * Social securities share of the invoice amount to be paid directly to the social securities collector.
   */
  ShiftedSocialSecurities = "SSS",
  /**
   * State/provincial sales tax
   *
   * All applicable sale taxes by authorities at the state or provincial level, below national level.
   */
  StateProvincialSalesTax = "STT",
  /**
   * Suspended duty
   *
   * Duty suspended or deferred from payment.
   */
  SuspendedDuty = "SUP",
  /**
   * Surtax
   *
   * A tax or duty applied on and in addition to existing duties and taxes.
   */
  Surtax = "SUR",
  /**
   * Shifted wage tax
   *
   * Wage tax share of the invoice amount to be paid directly to the tax collector(s office).
   */
  ShiftedWageTax = "SWT",
  /**
   * Alcohol mark tax
   *
   * A tax levied based on the type of alcohol being obtained.
   */
  AlcoholMarkTax = "TAC",
  /**
   * Total
   *
   * The summary amount of all taxes.
   */
  Total = "TOT",
  /**
   * Turnover tax
   *
   * Tax levied on the total sales/turnover of a corporation.
   */
  TurnoverTax = "TOX",
  /**
   * Tonnage taxes
   *
   * Tax levied based on the vessel's net tonnage.
   */
  TonnageTaxes = "TTA",
  /**
   * Valuation deposit
   *
   * Duty paid and held on deposit, by Customs, during an investigation period prior to a formal decision on valuation of the goods being made.
   */
  ValuationDeposit = "VAD",
  /**
   * Value added tax
   *
   * A tax on domestic or imported goods applied to the value added at each stage in the production/distribution cycle.
   */
  ValueAddedTax = "VAT",
}

export function description(value: VATCAT): string {
  switch (value) {
    case VATCAT.PetroleumTax:
      return "Petroleum tax";
    case VATCAT.ProvisionalCountervailingDutyCash:
      return "Provisional countervailing duty cash";
    case VATCAT.ProvisionalCountervailingDutyBond:
      return "Provisional countervailing duty bond";
    case VATCAT.TobaccoTax:
      return "Tobacco tax";
    case VATCAT.EnergyFee:
      return "Energy fee";
    case VATCAT.CoffeeTax:
      return "Coffee tax";
    case VATCAT.HarmonisedSalesTaxCanadian:
      return "Harmonised sales tax, Canadian";
    case VATCAT.QuebecSalesTax:
      return "Quebec sales tax";
    case VATCAT.CanadianProvincialSalesTax:
      return "Canadian provincial sales tax";
    case VATCAT.TaxOnReplacementPart:
      return "Tax on replacement part";
    case VATCAT.MineralOilTax:
      return "Mineral oil tax";
    case VATCAT.SpecialTax:
      return "Special tax";
    case VATCAT.InsuranceTax:
      return "Insurance tax";
    case VATCAT.ProvincialCannabisTax:
      return "Provincial Cannabis Tax";
    case VATCAT.OutstandingDutyInterest:
      return "Outstanding duty interest";
    case VATCAT.AntiDumpingDuty:
      return "Anti-dumping duty";
    case VATCAT.StampDutyImpostaDiBollo:
      return "Stamp duty (Imposta di Bollo)";
    case VATCAT.AgriculturalLevy:
      return "Agricultural levy";
    case VATCAT.CarTax:
      return "Car tax";
    case VATCAT.PaperConsortiumTaxItaly:
      return "Paper consortium tax (Italy)";
    case VATCAT.CommoditySpecificTax:
      return "Commodity specific tax";
    case VATCAT.CustomsDuty:
      return "Customs duty";
    case VATCAT.CountervailingDuty:
      return "Countervailing duty";
    case VATCAT.EnvironmentalTax:
      return "Environmental tax";
    case VATCAT.ExciseDuty:
      return "Excise duty";
    case VATCAT.AgriculturalExportRebate:
      return "Agricultural export rebate";
    case VATCAT.FederalExciseTax:
      return "Federal excise tax";
    case VATCAT.Free:
      return "Free";
    case VATCAT.GeneralConstructionTax:
      return "General construction tax";
    case VATCAT.GoodsAndServicesTax:
      return "Goods and services tax";
    case VATCAT.IlluminantsTax:
      return "Illuminants tax";
    case VATCAT.ImportTax:
      return "Import tax";
    case VATCAT.IndividualTax:
      return "Individual tax";
    case VATCAT.BusinessLicenseFee:
      return "Business license fee";
    case VATCAT.LocalConstructionTax:
      return "Local construction tax";
    case VATCAT.LightDuesPayable:
      return "Light dues payable";
    case VATCAT.LocalSalesTax:
      return "Local sales tax";
    case VATCAT.LustTax:
      return "Lust tax";
    case VATCAT.MonetaryCompensatoryAmount:
      return "Monetary compensatory amount";
    case VATCAT.MiscellaneousCashDeposit:
      return "Miscellaneous cash deposit";
    case VATCAT.OtherTaxes:
      return "Other taxes";
    case VATCAT.ProvisionalDutyBond:
      return "Provisional duty bond";
    case VATCAT.ProvisionalDutyCash:
      return "Provisional duty cash";
    case VATCAT.PreferenceDuty:
      return "Preference duty";
    case VATCAT.SpecialConstructionTax:
      return "Special construction tax";
    case VATCAT.ShiftedSocialSecurities:
      return "Shifted social securities";
    case VATCAT.StateProvincialSalesTax:
      return "State/provincial sales tax";
    case VATCAT.SuspendedDuty:
      return "Suspended duty";
    case VATCAT.Surtax:
      return "Surtax";
    case VATCAT.ShiftedWageTax:
      return "Shifted wage tax";
    case VATCAT.AlcoholMarkTax:
      return "Alcohol mark tax";
    case VATCAT.Total:
      return "Total";
    case VATCAT.TurnoverTax:
      return "Turnover tax";
    case VATCAT.TonnageTaxes:
      return "Tonnage taxes";
    case VATCAT.ValuationDeposit:
      return "Valuation deposit";
    case VATCAT.ValueAddedTax:
      return "Value added tax";
  }
}
