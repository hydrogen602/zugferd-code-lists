export enum MIME {
  /**
   * application/pdf
   */
  ApplicationPdf = "application/pdf",
  /**
   * image/png
   */
  ImagePng = "image/png",
  /**
   * image/jpeg
   */
  ImageJpeg = "image/jpeg",
  /**
   * text/csv
   */
  TextCsv = "text/csv",
  /**
   * application/vnd.openxmlformats-officedocument.spreadsheetml.sheet
   */
  ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
  /**
   * application/vnd.oasis.opendocument.spreadsheet
   */
  ApplicationVndOasisOpendocumentSpreadsheet = "application/vnd.oasis.opendocument.spreadsheet",
  /**
   * application/xml
   */
  ApplicationXml = "application/xml",
  /**
   * text/xml
   */
  TextXml = "text/xml",
}

export function description(value: MIME): string {
  switch (value) {
    case MIME.ApplicationPdf:
      return "application/pdf";
    case MIME.ImagePng:
      return "image/png";
    case MIME.ImageJpeg:
      return "image/jpeg";
    case MIME.TextCsv:
      return "text/csv";
    case MIME.ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet:
      return "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
    case MIME.ApplicationVndOasisOpendocumentSpreadsheet:
      return "application/vnd.oasis.opendocument.spreadsheet";
    case MIME.ApplicationXml:
      return "application/xml";
    case MIME.TextXml:
      return "text/xml";
  }
}
