use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SPInstruments {
    symbols: Vec<Instrument>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Instrument {
    symbol: String,
    name: String,
    sector: String,
}

pub fn get_symbols() -> Vec<String> {
    let content = r#"{ "symbols": [
    {
        "symbol": "MMM",
        "name": "3M",
        "sector": "Industrials"
    },
    {
        "symbol": "AOS",
        "name": "A. O. Smith",
        "sector": "Industrials"
    },
    {
        "symbol": "ABT",
        "name": "Abbott Laboratories",
        "sector": "Health Care"
    },
    {
        "symbol": "ABBV",
        "name": "AbbVie",
        "sector": "Health Care"
    },
    {
        "symbol": "ABMD",
        "name": "Abiomed",
        "sector": "Health Care"
    },
    {
        "symbol": "ACN",
        "name": "Accenture",
        "sector": "Information Technology"
    },
    {
        "symbol": "ATVI",
        "name": "Activision Blizzard",
        "sector": "Communication Services"
    },
    {
        "symbol": "ADM",
        "name": "ADM",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "ADBE",
        "name": "Adobe",
        "sector": "Information Technology"
    },
    {
        "symbol": "AAP",
        "name": "Advance Auto Parts",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "AMD",
        "name": "Advanced Micro Devices",
        "sector": "Information Technology"
    },
    {
        "symbol": "AES",
        "name": "AES Corp",
        "sector": "Utilities"
    },
    {
        "symbol": "AFL",
        "name": "Aflac",
        "sector": "Financials"
    },
    {
        "symbol": "A",
        "name": "Agilent Technologies",
        "sector": "Health Care"
    },
    {
        "symbol": "APD",
        "name": "Air Products & Chemicals",
        "sector": "Materials"
    },
    {
        "symbol": "AKAM",
        "name": "Akamai Technologies",
        "sector": "Information Technology"
    },
    {
        "symbol": "ALK",
        "name": "Alaska Air Group",
        "sector": "Industrials"
    },
    {
        "symbol": "ALB",
        "name": "Albemarle Corporation",
        "sector": "Materials"
    },
    {
        "symbol": "ARE",
        "name": "Alexandria Real Estate Equities",
        "sector": "Real Estate"
    },
    {
        "symbol": "ALGN",
        "name": "Align Technology",
        "sector": "Health Care"
    },
    {
        "symbol": "ALLE",
        "name": "Allegion",
        "sector": "Industrials"
    },
    {
        "symbol": "LNT",
        "name": "Alliant Energy",
        "sector": "Utilities"
    },
    {
        "symbol": "ALL",
        "name": "Allstate Corp",
        "sector": "Financials"
    },
    {
        "symbol": "GOOGL",
        "name": "Alphabet (Class A)",
        "sector": "Communication Services"
    },
    {
        "symbol": "GOOG",
        "name": "Alphabet (Class C)",
        "sector": "Communication Services"
    },
    {
        "symbol": "MO",
        "name": "Altria Group",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "AMZN",
        "name": "Amazon",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "AMCR",
        "name": "Amcor",
        "sector": "Materials"
    },
    {
        "symbol": "AEE",
        "name": "Ameren Corp",
        "sector": "Utilities"
    },
    {
        "symbol": "AAL",
        "name": "American Airlines Group",
        "sector": "Industrials"
    },
    {
        "symbol": "AEP",
        "name": "American Electric Power",
        "sector": "Utilities"
    },
    {
        "symbol": "AXP",
        "name": "American Express",
        "sector": "Financials"
    },
    {
        "symbol": "AIG",
        "name": "American International Group",
        "sector": "Financials"
    },
    {
        "symbol": "AMT",
        "name": "American Tower",
        "sector": "Real Estate"
    },
    {
        "symbol": "AWK",
        "name": "American Water Works",
        "sector": "Utilities"
    },
    {
        "symbol": "AMP",
        "name": "Ameriprise Financial",
        "sector": "Financials"
    },
    {
        "symbol": "ABC",
        "name": "AmerisourceBergen",
        "sector": "Health Care"
    },
    {
        "symbol": "AME",
        "name": "Ametek",
        "sector": "Industrials"
    },
    {
        "symbol": "AMGN",
        "name": "Amgen",
        "sector": "Health Care"
    },
    {
        "symbol": "APH",
        "name": "Amphenol",
        "sector": "Information Technology"
    },
    {
        "symbol": "ADI",
        "name": "Analog Devices",
        "sector": "Information Technology"
    },
    {
        "symbol": "ANSS",
        "name": "Ansys",
        "sector": "Information Technology"
    },
    {
        "symbol": "ANTM",
        "name": "Anthem",
        "sector": "Health Care"
    },
    {
        "symbol": "AON",
        "name": "Aon",
        "sector": "Financials"
    },
    {
        "symbol": "APA",
        "name": "APA Corporation",
        "sector": "Energy"
    },
    {
        "symbol": "AAPL",
        "name": "Apple",
        "sector": "Information Technology"
    },
    {
        "symbol": "AMAT",
        "name": "Applied Materials",
        "sector": "Information Technology"
    },
    {
        "symbol": "APTV",
        "name": "Aptiv",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "ANET",
        "name": "Arista Networks",
        "sector": "Information Technology"
    },
    {
        "symbol": "AJG",
        "name": "Arthur J. Gallagher & Co.",
        "sector": "Financials"
    },
    {
        "symbol": "AIZ",
        "name": "Assurant",
        "sector": "Financials"
    },
    {
        "symbol": "T",
        "name": "AT&T",
        "sector": "Communication Services"
    },
    {
        "symbol": "ATO",
        "name": "Atmos Energy",
        "sector": "Utilities"
    },
    {
        "symbol": "ADSK",
        "name": "Autodesk",
        "sector": "Information Technology"
    },
    {
        "symbol": "ADP",
        "name": "Automatic Data Processing",
        "sector": "Information Technology"
    },
    {
        "symbol": "AZO",
        "name": "AutoZone",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "AVB",
        "name": "AvalonBay Communities",
        "sector": "Real Estate"
    },
    {
        "symbol": "AVY",
        "name": "Avery Dennison",
        "sector": "Materials"
    },
    {
        "symbol": "BKR",
        "name": "Baker Hughes",
        "sector": "Energy"
    },
    {
        "symbol": "BLL",
        "name": "Ball Corp",
        "sector": "Materials"
    },
    {
        "symbol": "BAC",
        "name": "Bank of America",
        "sector": "Financials"
    },
    {
        "symbol": "BBWI",
        "name": "Bath & Body Works Inc.",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "BAX",
        "name": "Baxter International",
        "sector": "Health Care"
    },
    {
        "symbol": "BDX",
        "name": "Becton Dickinson",
        "sector": "Health Care"
    },
    {
        "symbol": "BRK.B",
        "name": "Berkshire Hathaway",
        "sector": "Financials"
    },
    {
        "symbol": "BBY",
        "name": "Best Buy",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "BIO",
        "name": "Bio-Rad Laboratories",
        "sector": "Health Care"
    },
    {
        "symbol": "TECH",
        "name": "Bio-Techne",
        "sector": "Health Care"
    },
    {
        "symbol": "BIIB",
        "name": "Biogen",
        "sector": "Health Care"
    },
    {
        "symbol": "BLK",
        "name": "BlackRock",
        "sector": "Financials"
    },
    {
        "symbol": "BK",
        "name": "BNY Mellon",
        "sector": "Financials"
    },
    {
        "symbol": "BA",
        "name": "Boeing",
        "sector": "Industrials"
    },
    {
        "symbol": "BKNG",
        "name": "Booking Holdings",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "BWA",
        "name": "BorgWarner",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "BXP",
        "name": "Boston Properties",
        "sector": "Real Estate"
    },
    {
        "symbol": "BSX",
        "name": "Boston Scientific",
        "sector": "Health Care"
    },
    {
        "symbol": "BMY",
        "name": "Bristol Myers Squibb",
        "sector": "Health Care"
    },
    {
        "symbol": "AVGO",
        "name": "Broadcom",
        "sector": "Information Technology"
    },
    {
        "symbol": "BR",
        "name": "Broadridge Financial Solutions",
        "sector": "Information Technology"
    },
    {
        "symbol": "BRO",
        "name": "Brown & Brown",
        "sector": "Financials"
    },
    {
        "symbol": "BF.B",
        "name": "Brown–Forman",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "CHRW",
        "name": "C. H. Robinson",
        "sector": "Industrials"
    },
    {
        "symbol": "CDNS",
        "name": "Cadence Design Systems",
        "sector": "Information Technology"
    },
    {
        "symbol": "CZR",
        "name": "Caesars Entertainment",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "CPB",
        "name": "Campbell Soup",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "COF",
        "name": "Capital One Financial",
        "sector": "Financials"
    },
    {
        "symbol": "CAH",
        "name": "Cardinal Health",
        "sector": "Health Care"
    },
    {
        "symbol": "KMX",
        "name": "CarMax",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "CCL",
        "name": "Carnival Corporation",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "CARR",
        "name": "Carrier Global",
        "sector": "Industrials"
    },
    {
        "symbol": "CTLT",
        "name": "Catalent",
        "sector": "Health Care"
    },
    {
        "symbol": "CAT",
        "name": "Caterpillar",
        "sector": "Industrials"
    },
    {
        "symbol": "CBOE",
        "name": "Cboe Global Markets",
        "sector": "Financials"
    },
    {
        "symbol": "CBRE",
        "name": "CBRE",
        "sector": "Real Estate"
    },
    {
        "symbol": "CDW",
        "name": "CDW",
        "sector": "Information Technology"
    },
    {
        "symbol": "CE",
        "name": "Celanese",
        "sector": "Materials"
    },
    {
        "symbol": "CNC",
        "name": "Centene Corporation",
        "sector": "Health Care"
    },
    {
        "symbol": "CNP",
        "name": "CenterPoint Energy",
        "sector": "Utilities"
    },
    {
        "symbol": "CDAY",
        "name": "Ceridian",
        "sector": "Information Technology"
    },
    {
        "symbol": "CERN",
        "name": "Cerner",
        "sector": "Health Care"
    },
    {
        "symbol": "CF",
        "name": "CF Industries",
        "sector": "Materials"
    },
    {
        "symbol": "CRL",
        "name": "Charles River Laboratories",
        "sector": "Health Care"
    },
    {
        "symbol": "SCHW",
        "name": "Charles Schwab Corporation",
        "sector": "Financials"
    },
    {
        "symbol": "CHTR",
        "name": "Charter Communications",
        "sector": "Communication Services"
    },
    {
        "symbol": "CVX",
        "name": "Chevron Corporation",
        "sector": "Energy"
    },
    {
        "symbol": "CMG",
        "name": "Chipotle Mexican Grill",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "CB",
        "name": "Chubb",
        "sector": "Financials"
    },
    {
        "symbol": "CHD",
        "name": "Church & Dwight",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "CI",
        "name": "Cigna",
        "sector": "Health Care"
    },
    {
        "symbol": "CINF",
        "name": "Cincinnati Financial",
        "sector": "Financials"
    },
    {
        "symbol": "CTAS",
        "name": "Cintas Corporation",
        "sector": "Industrials"
    },
    {
        "symbol": "CSCO",
        "name": "Cisco Systems",
        "sector": "Information Technology"
    },
    {
        "symbol": "C",
        "name": "Citigroup",
        "sector": "Financials"
    },
    {
        "symbol": "CFG",
        "name": "Citizens Financial Group",
        "sector": "Financials"
    },
    {
        "symbol": "CTXS",
        "name": "Citrix Systems",
        "sector": "Information Technology"
    },
    {
        "symbol": "CLX",
        "name": "Clorox",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "CME",
        "name": "CME Group",
        "sector": "Financials"
    },
    {
        "symbol": "CMS",
        "name": "CMS Energy",
        "sector": "Utilities"
    },
    {
        "symbol": "KO",
        "name": "Coca-Cola Company",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "CTSH",
        "name": "Cognizant Technology Solutions",
        "sector": "Information Technology"
    },
    {
        "symbol": "CL",
        "name": "Colgate-Palmolive",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "CMCSA",
        "name": "Comcast",
        "sector": "Communication Services"
    },
    {
        "symbol": "CMA",
        "name": "Comerica",
        "sector": "Financials"
    },
    {
        "symbol": "CAG",
        "name": "Conagra Brands",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "COP",
        "name": "ConocoPhillips",
        "sector": "Energy"
    },
    {
        "symbol": "ED",
        "name": "Consolidated Edison",
        "sector": "Utilities"
    },
    {
        "symbol": "STZ",
        "name": "Constellation Brands",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "CPRT",
        "name": "Copart",
        "sector": "Industrials"
    },
    {
        "symbol": "GLW",
        "name": "Corning",
        "sector": "Information Technology"
    },
    {
        "symbol": "CTVA",
        "name": "Corteva",
        "sector": "Materials"
    },
    {
        "symbol": "COST",
        "name": "Costco",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "CTRA",
        "name": "Coterra",
        "sector": "Energy"
    },
    {
        "symbol": "CCI",
        "name": "Crown Castle",
        "sector": "Real Estate"
    },
    {
        "symbol": "CSX",
        "name": "CSX",
        "sector": "Industrials"
    },
    {
        "symbol": "CMI",
        "name": "Cummins",
        "sector": "Industrials"
    },
    {
        "symbol": "CVS",
        "name": "CVS Health",
        "sector": "Health Care"
    },
    {
        "symbol": "DHI",
        "name": "D. R. Horton",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "DHR",
        "name": "Danaher Corporation",
        "sector": "Health Care"
    },
    {
        "symbol": "DRI",
        "name": "Darden Restaurants",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "DVA",
        "name": "DaVita",
        "sector": "Health Care"
    },
    {
        "symbol": "DE",
        "name": "Deere & Co.",
        "sector": "Industrials"
    },
    {
        "symbol": "DAL",
        "name": "Delta Air Lines",
        "sector": "Industrials"
    },
    {
        "symbol": "XRAY",
        "name": "Dentsply Sirona",
        "sector": "Health Care"
    },
    {
        "symbol": "DVN",
        "name": "Devon Energy",
        "sector": "Energy"
    },
    {
        "symbol": "DXCM",
        "name": "DexCom",
        "sector": "Health Care"
    },
    {
        "symbol": "FANG",
        "name": "Diamondback Energy",
        "sector": "Energy"
    },
    {
        "symbol": "DLR",
        "name": "Digital Realty Trust",
        "sector": "Real Estate"
    },
    {
        "symbol": "DFS",
        "name": "Discover Financial Services",
        "sector": "Financials"
    },
    {
        "symbol": "DISCA",
        "name": "Discovery (Series A)",
        "sector": "Communication Services"
    },
    {
        "symbol": "DISCK",
        "name": "Discovery (Series C)",
        "sector": "Communication Services"
    },
    {
        "symbol": "DISH",
        "name": "Dish Network",
        "sector": "Communication Services"
    },
    {
        "symbol": "DG",
        "name": "Dollar General",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "DLTR",
        "name": "Dollar Tree",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "D",
        "name": "Dominion Energy",
        "sector": "Utilities"
    },
    {
        "symbol": "DPZ",
        "name": "Domino's Pizza",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "DOV",
        "name": "Dover Corporation",
        "sector": "Industrials"
    },
    {
        "symbol": "DOW",
        "name": "Dow",
        "sector": "Materials"
    },
    {
        "symbol": "DTE",
        "name": "DTE Energy",
        "sector": "Utilities"
    },
    {
        "symbol": "DUK",
        "name": "Duke Energy",
        "sector": "Utilities"
    },
    {
        "symbol": "DRE",
        "name": "Duke Realty Corp",
        "sector": "Real Estate"
    },
    {
        "symbol": "DD",
        "name": "DuPont",
        "sector": "Materials"
    },
    {
        "symbol": "DXC",
        "name": "DXC Technology",
        "sector": "Information Technology"
    },
    {
        "symbol": "EMN",
        "name": "Eastman Chemical",
        "sector": "Materials"
    },
    {
        "symbol": "ETN",
        "name": "Eaton Corporation",
        "sector": "Industrials"
    },
    {
        "symbol": "EBAY",
        "name": "eBay",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "ECL",
        "name": "Ecolab",
        "sector": "Materials"
    },
    {
        "symbol": "EIX",
        "name": "Edison International",
        "sector": "Utilities"
    },
    {
        "symbol": "EW",
        "name": "Edwards Lifesciences",
        "sector": "Health Care"
    },
    {
        "symbol": "EA",
        "name": "Electronic Arts",
        "sector": "Communication Services"
    },
    {
        "symbol": "LLY",
        "name": "Eli Lilly & Co",
        "sector": "Health Care"
    },
    {
        "symbol": "EMR",
        "name": "Emerson Electric Company",
        "sector": "Industrials"
    },
    {
        "symbol": "ENPH",
        "name": "Enphase Energy",
        "sector": "Information Technology"
    },
    {
        "symbol": "ETR",
        "name": "Entergy",
        "sector": "Utilities"
    },
    {
        "symbol": "EOG",
        "name": "EOG Resources",
        "sector": "Energy"
    },
    {
        "symbol": "EFX",
        "name": "Equifax",
        "sector": "Industrials"
    },
    {
        "symbol": "EQIX",
        "name": "Equinix",
        "sector": "Real Estate"
    },
    {
        "symbol": "EQR",
        "name": "Equity Residential",
        "sector": "Real Estate"
    },
    {
        "symbol": "ESS",
        "name": "Essex Property Trust",
        "sector": "Real Estate"
    },
    {
        "symbol": "EL",
        "name": "Estée Lauder Companies",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "ETSY",
        "name": "Etsy",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "RE",
        "name": "Everest Re",
        "sector": "Financials"
    },
    {
        "symbol": "EVRG",
        "name": "Evergy",
        "sector": "Utilities"
    },
    {
        "symbol": "ES",
        "name": "Eversource Energy",
        "sector": "Utilities"
    },
    {
        "symbol": "EXC",
        "name": "Exelon",
        "sector": "Utilities"
    },
    {
        "symbol": "EXPE",
        "name": "Expedia Group",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "EXPD",
        "name": "Expeditors",
        "sector": "Industrials"
    },
    {
        "symbol": "EXR",
        "name": "Extra Space Storage",
        "sector": "Real Estate"
    },
    {
        "symbol": "XOM",
        "name": "ExxonMobil",
        "sector": "Energy"
    },
    {
        "symbol": "FFIV",
        "name": "F5 Networks",
        "sector": "Information Technology"
    },
    {
        "symbol": "FB",
        "name": "Facebook",
        "sector": "Communication Services"
    },
    {
        "symbol": "FAST",
        "name": "Fastenal",
        "sector": "Industrials"
    },
    {
        "symbol": "FRT",
        "name": "Federal Realty Investment Trust",
        "sector": "Real Estate"
    },
    {
        "symbol": "FDX",
        "name": "FedEx",
        "sector": "Industrials"
    },
    {
        "symbol": "FIS",
        "name": "Fidelity National Information Services",
        "sector": "Information Technology"
    },
    {
        "symbol": "FITB",
        "name": "Fifth Third Bancorp",
        "sector": "Financials"
    },
    {
        "symbol": "FRC",
        "name": "First Republic Bank",
        "sector": "Financials"
    },
    {
        "symbol": "FE",
        "name": "FirstEnergy",
        "sector": "Utilities"
    },
    {
        "symbol": "FISV",
        "name": "Fiserv",
        "sector": "Information Technology"
    },
    {
        "symbol": "FLT",
        "name": "Fleetcor",
        "sector": "Information Technology"
    },
    {
        "symbol": "FMC",
        "name": "FMC Corporation",
        "sector": "Materials"
    },
    {
        "symbol": "F",
        "name": "Ford",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "FTNT",
        "name": "Fortinet",
        "sector": "Information Technology"
    },
    {
        "symbol": "FTV",
        "name": "Fortive",
        "sector": "Industrials"
    },
    {
        "symbol": "FBHS",
        "name": "Fortune Brands Home & Security",
        "sector": "Industrials"
    },
    {
        "symbol": "FOXA",
        "name": "Fox Corporation (Class A)",
        "sector": "Communication Services"
    },
    {
        "symbol": "FOX",
        "name": "Fox Corporation (Class B)",
        "sector": "Communication Services"
    },
    {
        "symbol": "BEN",
        "name": "Franklin Resources",
        "sector": "Financials"
    },
    {
        "symbol": "FCX",
        "name": "Freeport-McMoRan",
        "sector": "Materials"
    },
    {
        "symbol": "GPS",
        "name": "Gap",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "GRMN",
        "name": "Garmin",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "IT",
        "name": "Gartner",
        "sector": "Information Technology"
    },
    {
        "symbol": "GNRC",
        "name": "Generac Holdings",
        "sector": "Industrials"
    },
    {
        "symbol": "GD",
        "name": "General Dynamics",
        "sector": "Industrials"
    },
    {
        "symbol": "GE",
        "name": "General Electric",
        "sector": "Industrials"
    },
    {
        "symbol": "GIS",
        "name": "General Mills",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "GM",
        "name": "General Motors",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "GPC",
        "name": "Genuine Parts",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "GILD",
        "name": "Gilead Sciences",
        "sector": "Health Care"
    },
    {
        "symbol": "GPN",
        "name": "Global Payments",
        "sector": "Information Technology"
    },
    {
        "symbol": "GL",
        "name": "Globe Life",
        "sector": "Financials"
    },
    {
        "symbol": "GS",
        "name": "Goldman Sachs",
        "sector": "Financials"
    },
    {
        "symbol": "HAL",
        "name": "Halliburton",
        "sector": "Energy"
    },
    {
        "symbol": "HBI",
        "name": "Hanesbrands",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "HAS",
        "name": "Hasbro",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "HCA",
        "name": "HCA Healthcare",
        "sector": "Health Care"
    },
    {
        "symbol": "PEAK",
        "name": "Healthpeak Properties",
        "sector": "Real Estate"
    },
    {
        "symbol": "HSIC",
        "name": "Henry Schein",
        "sector": "Health Care"
    },
    {
        "symbol": "HES",
        "name": "Hess Corporation",
        "sector": "Energy"
    },
    {
        "symbol": "HPE",
        "name": "Hewlett Packard Enterprise",
        "sector": "Information Technology"
    },
    {
        "symbol": "HLT",
        "name": "Hilton Worldwide",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "HOLX",
        "name": "Hologic",
        "sector": "Health Care"
    },
    {
        "symbol": "HD",
        "name": "Home Depot",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "HON",
        "name": "Honeywell",
        "sector": "Industrials"
    },
    {
        "symbol": "HRL",
        "name": "Hormel",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "HST",
        "name": "Host Hotels & Resorts",
        "sector": "Real Estate"
    },
    {
        "symbol": "HWM",
        "name": "Howmet Aerospace",
        "sector": "Industrials"
    },
    {
        "symbol": "HPQ",
        "name": "HP",
        "sector": "Information Technology"
    },
    {
        "symbol": "HUM",
        "name": "Humana",
        "sector": "Health Care"
    },
    {
        "symbol": "HBAN",
        "name": "Huntington Bancshares",
        "sector": "Financials"
    },
    {
        "symbol": "HII",
        "name": "Huntington Ingalls Industries",
        "sector": "Industrials"
    },
    {
        "symbol": "IBM",
        "name": "IBM",
        "sector": "Information Technology"
    },
    {
        "symbol": "IEX",
        "name": "IDEX Corporation",
        "sector": "Industrials"
    },
    {
        "symbol": "IDXX",
        "name": "Idexx Laboratories",
        "sector": "Health Care"
    },
    {
        "symbol": "INFO",
        "name": "IHS Markit",
        "sector": "Industrials"
    },
    {
        "symbol": "ITW",
        "name": "Illinois Tool Works",
        "sector": "Industrials"
    },
    {
        "symbol": "ILMN",
        "name": "Illumina",
        "sector": "Health Care"
    },
    {
        "symbol": "INCY",
        "name": "Incyte",
        "sector": "Health Care"
    },
    {
        "symbol": "IR",
        "name": "Ingersoll Rand",
        "sector": "Industrials"
    },
    {
        "symbol": "INTC",
        "name": "Intel",
        "sector": "Information Technology"
    },
    {
        "symbol": "ICE",
        "name": "Intercontinental Exchange",
        "sector": "Financials"
    },
    {
        "symbol": "IFF",
        "name": "International Flavors & Fragrances",
        "sector": "Materials"
    },
    {
        "symbol": "IP",
        "name": "International Paper",
        "sector": "Materials"
    },
    {
        "symbol": "IPG",
        "name": "Interpublic Group",
        "sector": "Communication Services"
    },
    {
        "symbol": "INTU",
        "name": "Intuit",
        "sector": "Information Technology"
    },
    {
        "symbol": "ISRG",
        "name": "Intuitive Surgical",
        "sector": "Health Care"
    },
    {
        "symbol": "IVZ",
        "name": "Invesco",
        "sector": "Financials"
    },
    {
        "symbol": "IPGP",
        "name": "IPG Photonics",
        "sector": "Information Technology"
    },
    {
        "symbol": "IQV",
        "name": "IQVIA",
        "sector": "Health Care"
    },
    {
        "symbol": "IRM",
        "name": "Iron Mountain",
        "sector": "Real Estate"
    },
    {
        "symbol": "JBHT",
        "name": "J. B. Hunt",
        "sector": "Industrials"
    },
    {
        "symbol": "JKHY",
        "name": "Jack Henry & Associates",
        "sector": "Information Technology"
    },
    {
        "symbol": "J",
        "name": "Jacobs Engineering Group",
        "sector": "Industrials"
    },
    {
        "symbol": "SJM",
        "name": "JM Smucker",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "JNJ",
        "name": "Johnson & Johnson",
        "sector": "Health Care"
    },
    {
        "symbol": "JCI",
        "name": "Johnson Controls",
        "sector": "Industrials"
    },
    {
        "symbol": "JPM",
        "name": "JPMorgan Chase",
        "sector": "Financials"
    },
    {
        "symbol": "JNPR",
        "name": "Juniper Networks",
        "sector": "Information Technology"
    },
    {
        "symbol": "KSU",
        "name": "Kansas City Southern",
        "sector": "Industrials"
    },
    {
        "symbol": "K",
        "name": "Kellogg's",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "KEY",
        "name": "KeyCorp",
        "sector": "Financials"
    },
    {
        "symbol": "KEYS",
        "name": "Keysight Technologies",
        "sector": "Information Technology"
    },
    {
        "symbol": "KMB",
        "name": "Kimberly-Clark",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "KIM",
        "name": "Kimco Realty",
        "sector": "Real Estate"
    },
    {
        "symbol": "KMI",
        "name": "Kinder Morgan",
        "sector": "Energy"
    },
    {
        "symbol": "KLAC",
        "name": "KLA Corporation",
        "sector": "Information Technology"
    },
    {
        "symbol": "KHC",
        "name": "Kraft Heinz",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "KR",
        "name": "Kroger",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "LHX",
        "name": "L3Harris Technologies",
        "sector": "Industrials"
    },
    {
        "symbol": "LH",
        "name": "LabCorp",
        "sector": "Health Care"
    },
    {
        "symbol": "LRCX",
        "name": "Lam Research",
        "sector": "Information Technology"
    },
    {
        "symbol": "LW",
        "name": "Lamb Weston",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "LVS",
        "name": "Las Vegas Sands",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "LEG",
        "name": "Leggett & Platt",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "LDOS",
        "name": "Leidos",
        "sector": "Industrials"
    },
    {
        "symbol": "LEN",
        "name": "Lennar",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "LNC",
        "name": "Lincoln National",
        "sector": "Financials"
    },
    {
        "symbol": "LIN",
        "name": "Linde",
        "sector": "Materials"
    },
    {
        "symbol": "LYV",
        "name": "Live Nation Entertainment",
        "sector": "Communication Services"
    },
    {
        "symbol": "LKQ",
        "name": "LKQ Corporation",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "LMT",
        "name": "Lockheed Martin",
        "sector": "Industrials"
    },
    {
        "symbol": "L",
        "name": "Loews Corporation",
        "sector": "Financials"
    },
    {
        "symbol": "LOW",
        "name": "Lowe's",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "LUMN",
        "name": "Lumen Technologies",
        "sector": "Communication Services"
    },
    {
        "symbol": "LYB",
        "name": "LyondellBasell",
        "sector": "Materials"
    },
    {
        "symbol": "MTB",
        "name": "M&T Bank",
        "sector": "Financials"
    },
    {
        "symbol": "MRO",
        "name": "Marathon Oil",
        "sector": "Energy"
    },
    {
        "symbol": "MPC",
        "name": "Marathon Petroleum",
        "sector": "Energy"
    },
    {
        "symbol": "MKTX",
        "name": "MarketAxess",
        "sector": "Financials"
    },
    {
        "symbol": "MAR",
        "name": "Marriott International",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "MMC",
        "name": "Marsh & McLennan",
        "sector": "Financials"
    },
    {
        "symbol": "MLM",
        "name": "Martin Marietta Materials",
        "sector": "Materials"
    },
    {
        "symbol": "MAS",
        "name": "Masco",
        "sector": "Industrials"
    },
    {
        "symbol": "MA",
        "name": "Mastercard",
        "sector": "Information Technology"
    },
    {
        "symbol": "MTCH",
        "name": "Match Group",
        "sector": "Communication Services"
    },
    {
        "symbol": "MKC",
        "name": "McCormick & Company",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "MCD",
        "name": "McDonald's",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "MCK",
        "name": "McKesson Corporation",
        "sector": "Health Care"
    },
    {
        "symbol": "MDT",
        "name": "Medtronic",
        "sector": "Health Care"
    },
    {
        "symbol": "MRK",
        "name": "Merck & Co.",
        "sector": "Health Care"
    },
    {
        "symbol": "MET",
        "name": "MetLife",
        "sector": "Financials"
    },
    {
        "symbol": "MTD",
        "name": "Mettler Toledo",
        "sector": "Health Care"
    },
    {
        "symbol": "MGM",
        "name": "MGM Resorts International",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "MCHP",
        "name": "Microchip Technology",
        "sector": "Information Technology"
    },
    {
        "symbol": "MU",
        "name": "Micron Technology",
        "sector": "Information Technology"
    },
    {
        "symbol": "MSFT",
        "name": "Microsoft",
        "sector": "Information Technology"
    },
    {
        "symbol": "MAA",
        "name": "Mid-America Apartments",
        "sector": "Real Estate"
    },
    {
        "symbol": "MRNA",
        "name": "Moderna",
        "sector": "Health Care"
    },
    {
        "symbol": "MHK",
        "name": "Mohawk Industries",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "TAP",
        "name": "Molson Coors Beverage Company",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "MDLZ",
        "name": "Mondelez International",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "MPWR",
        "name": "Monolithic Power Systems",
        "sector": "Information Technology"
    },
    {
        "symbol": "MNST",
        "name": "Monster Beverage",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "MCO",
        "name": "Moody's Corporation",
        "sector": "Financials"
    },
    {
        "symbol": "MS",
        "name": "Morgan Stanley",
        "sector": "Financials"
    },
    {
        "symbol": "MSI",
        "name": "Motorola Solutions",
        "sector": "Information Technology"
    },
    {
        "symbol": "MSCI",
        "name": "MSCI",
        "sector": "Financials"
    },
    {
        "symbol": "NDAQ",
        "name": "Nasdaq",
        "sector": "Financials"
    },
    {
        "symbol": "NTAP",
        "name": "NetApp",
        "sector": "Information Technology"
    },
    {
        "symbol": "NFLX",
        "name": "Netflix",
        "sector": "Communication Services"
    },
    {
        "symbol": "NWL",
        "name": "Newell Brands",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "NEM",
        "name": "Newmont",
        "sector": "Materials"
    },
    {
        "symbol": "NWSA",
        "name": "News Corp (Class A)",
        "sector": "Communication Services"
    },
    {
        "symbol": "NWS",
        "name": "News Corp (Class B)",
        "sector": "Communication Services"
    },
    {
        "symbol": "NEE",
        "name": "NextEra Energy",
        "sector": "Utilities"
    },
    {
        "symbol": "NLSN",
        "name": "Nielsen Holdings",
        "sector": "Industrials"
    },
    {
        "symbol": "NKE",
        "name": "Nike",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "NI",
        "name": "NiSource",
        "sector": "Utilities"
    },
    {
        "symbol": "NSC",
        "name": "Norfolk Southern",
        "sector": "Industrials"
    },
    {
        "symbol": "NTRS",
        "name": "Northern Trust",
        "sector": "Financials"
    },
    {
        "symbol": "NOC",
        "name": "Northrop Grumman",
        "sector": "Industrials"
    },
    {
        "symbol": "NLOK",
        "name": "NortonLifeLock",
        "sector": "Information Technology"
    },
    {
        "symbol": "NCLH",
        "name": "Norwegian Cruise Line Holdings",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "NRG",
        "name": "NRG Energy",
        "sector": "Utilities"
    },
    {
        "symbol": "NUE",
        "name": "Nucor",
        "sector": "Materials"
    },
    {
        "symbol": "NVDA",
        "name": "Nvidia",
        "sector": "Information Technology"
    },
    {
        "symbol": "NVR",
        "name": "NVR",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "NXPI",
        "name": "NXP",
        "sector": "Information Technology"
    },
    {
        "symbol": "ORLY",
        "name": "O'Reilly Automotive",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "OXY",
        "name": "Occidental Petroleum",
        "sector": "Energy"
    },
    {
        "symbol": "ODFL",
        "name": "Old Dominion Freight Line",
        "sector": "Industrials"
    },
    {
        "symbol": "OMC",
        "name": "Omnicom Group",
        "sector": "Communication Services"
    },
    {
        "symbol": "OKE",
        "name": "Oneok",
        "sector": "Energy"
    },
    {
        "symbol": "ORCL",
        "name": "Oracle",
        "sector": "Information Technology"
    },
    {
        "symbol": "OGN",
        "name": "Organon & Co.",
        "sector": "Health Care"
    },
    {
        "symbol": "OTIS",
        "name": "Otis Worldwide",
        "sector": "Industrials"
    },
    {
        "symbol": "PCAR",
        "name": "Paccar",
        "sector": "Industrials"
    },
    {
        "symbol": "PKG",
        "name": "Packaging Corporation of America",
        "sector": "Materials"
    },
    {
        "symbol": "PH",
        "name": "Parker-Hannifin",
        "sector": "Industrials"
    },
    {
        "symbol": "PAYX",
        "name": "Paychex",
        "sector": "Information Technology"
    },
    {
        "symbol": "PAYC",
        "name": "Paycom",
        "sector": "Information Technology"
    },
    {
        "symbol": "PYPL",
        "name": "PayPal",
        "sector": "Information Technology"
    },
    {
        "symbol": "PENN",
        "name": "Penn National Gaming",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "PNR",
        "name": "Pentair",
        "sector": "Industrials"
    },
    {
        "symbol": "PBCT",
        "name": "People's United Financial",
        "sector": "Financials"
    },
    {
        "symbol": "PEP",
        "name": "PepsiCo",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "PKI",
        "name": "PerkinElmer",
        "sector": "Health Care"
    },
    {
        "symbol": "PFE",
        "name": "Pfizer",
        "sector": "Health Care"
    },
    {
        "symbol": "PM",
        "name": "Philip Morris International",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "PSX",
        "name": "Phillips 66",
        "sector": "Energy"
    },
    {
        "symbol": "PNW",
        "name": "Pinnacle West Capital",
        "sector": "Utilities"
    },
    {
        "symbol": "PXD",
        "name": "Pioneer Natural Resources",
        "sector": "Energy"
    },
    {
        "symbol": "PNC",
        "name": "PNC Financial Services",
        "sector": "Financials"
    },
    {
        "symbol": "POOL",
        "name": "Pool Corporation",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "PPG",
        "name": "PPG Industries",
        "sector": "Materials"
    },
    {
        "symbol": "PPL",
        "name": "PPL",
        "sector": "Utilities"
    },
    {
        "symbol": "PFG",
        "name": "Principal Financial Group",
        "sector": "Financials"
    },
    {
        "symbol": "PG",
        "name": "Procter & Gamble",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "PGR",
        "name": "Progressive Corporation",
        "sector": "Financials"
    },
    {
        "symbol": "PLD",
        "name": "Prologis",
        "sector": "Real Estate"
    },
    {
        "symbol": "PRU",
        "name": "Prudential Financial",
        "sector": "Financials"
    },
    {
        "symbol": "PTC",
        "name": "PTC",
        "sector": "Information Technology"
    },
    {
        "symbol": "PEG",
        "name": "Public Service Enterprise Group",
        "sector": "Utilities"
    },
    {
        "symbol": "PSA",
        "name": "Public Storage",
        "sector": "Real Estate"
    },
    {
        "symbol": "PHM",
        "name": "PulteGroup",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "PVH",
        "name": "PVH",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "QRVO",
        "name": "Qorvo",
        "sector": "Information Technology"
    },
    {
        "symbol": "QCOM",
        "name": "Qualcomm",
        "sector": "Information Technology"
    },
    {
        "symbol": "PWR",
        "name": "Quanta Services",
        "sector": "Industrials"
    },
    {
        "symbol": "DGX",
        "name": "Quest Diagnostics",
        "sector": "Health Care"
    },
    {
        "symbol": "RL",
        "name": "Ralph Lauren Corporation",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "RJF",
        "name": "Raymond James Financial",
        "sector": "Financials"
    },
    {
        "symbol": "RTX",
        "name": "Raytheon Technologies",
        "sector": "Industrials"
    },
    {
        "symbol": "O",
        "name": "Realty Income Corporation",
        "sector": "Real Estate"
    },
    {
        "symbol": "REG",
        "name": "Regency Centers",
        "sector": "Real Estate"
    },
    {
        "symbol": "REGN",
        "name": "Regeneron Pharmaceuticals",
        "sector": "Health Care"
    },
    {
        "symbol": "RF",
        "name": "Regions Financial Corporation",
        "sector": "Financials"
    },
    {
        "symbol": "RSG",
        "name": "Republic Services",
        "sector": "Industrials"
    },
    {
        "symbol": "RMD",
        "name": "ResMed",
        "sector": "Health Care"
    },
    {
        "symbol": "RHI",
        "name": "Robert Half International",
        "sector": "Industrials"
    },
    {
        "symbol": "ROK",
        "name": "Rockwell Automation",
        "sector": "Industrials"
    },
    {
        "symbol": "ROL",
        "name": "Rollins",
        "sector": "Industrials"
    },
    {
        "symbol": "ROP",
        "name": "Roper Technologies",
        "sector": "Industrials"
    },
    {
        "symbol": "ROST",
        "name": "Ross Stores",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "RCL",
        "name": "Royal Caribbean Group",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "SPGI",
        "name": "S&P Global",
        "sector": "Financials"
    },
    {
        "symbol": "CRM",
        "name": "Salesforce",
        "sector": "Information Technology"
    },
    {
        "symbol": "SBAC",
        "name": "SBA Communications",
        "sector": "Real Estate"
    },
    {
        "symbol": "SLB",
        "name": "Schlumberger",
        "sector": "Energy"
    },
    {
        "symbol": "STX",
        "name": "Seagate Technology",
        "sector": "Information Technology"
    },
    {
        "symbol": "SEE",
        "name": "Sealed Air",
        "sector": "Materials"
    },
    {
        "symbol": "SRE",
        "name": "Sempra Energy",
        "sector": "Utilities"
    },
    {
        "symbol": "NOW",
        "name": "ServiceNow",
        "sector": "Information Technology"
    },
    {
        "symbol": "SHW",
        "name": "Sherwin-Williams",
        "sector": "Materials"
    },
    {
        "symbol": "SPG",
        "name": "Simon Property Group",
        "sector": "Real Estate"
    },
    {
        "symbol": "SWKS",
        "name": "Skyworks Solutions",
        "sector": "Information Technology"
    },
    {
        "symbol": "SNA",
        "name": "Snap-on",
        "sector": "Industrials"
    },
    {
        "symbol": "SO",
        "name": "Southern Company",
        "sector": "Utilities"
    },
    {
        "symbol": "LUV",
        "name": "Southwest Airlines",
        "sector": "Industrials"
    },
    {
        "symbol": "SWK",
        "name": "Stanley Black & Decker",
        "sector": "Industrials"
    },
    {
        "symbol": "SBUX",
        "name": "Starbucks",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "STT",
        "name": "State Street Corporation",
        "sector": "Financials"
    },
    {
        "symbol": "STE",
        "name": "Steris",
        "sector": "Health Care"
    },
    {
        "symbol": "SYK",
        "name": "Stryker Corporation",
        "sector": "Health Care"
    },
    {
        "symbol": "SIVB",
        "name": "SVB Financial",
        "sector": "Financials"
    },
    {
        "symbol": "SYF",
        "name": "Synchrony Financial",
        "sector": "Financials"
    },
    {
        "symbol": "SNPS",
        "name": "Synopsys",
        "sector": "Information Technology"
    },
    {
        "symbol": "SYY",
        "name": "Sysco",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "TMUS",
        "name": "T-Mobile US",
        "sector": "Communication Services"
    },
    {
        "symbol": "TROW",
        "name": "T. Rowe Price",
        "sector": "Financials"
    },
    {
        "symbol": "TTWO",
        "name": "Take-Two Interactive",
        "sector": "Communication Services"
    },
    {
        "symbol": "TPR",
        "name": "Tapestry",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "TGT",
        "name": "Target Corporation",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "TEL",
        "name": "TE Connectivity",
        "sector": "Information Technology"
    },
    {
        "symbol": "TDY",
        "name": "Teledyne Technologies",
        "sector": "Industrials"
    },
    {
        "symbol": "TFX",
        "name": "Teleflex",
        "sector": "Health Care"
    },
    {
        "symbol": "TER",
        "name": "Teradyne",
        "sector": "Information Technology"
    },
    {
        "symbol": "TSLA",
        "name": "Tesla",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "TXN",
        "name": "Texas Instruments",
        "sector": "Information Technology"
    },
    {
        "symbol": "TXT",
        "name": "Textron",
        "sector": "Industrials"
    },
    {
        "symbol": "COO",
        "name": "The Cooper Companies",
        "sector": "Health Care"
    },
    {
        "symbol": "HIG",
        "name": "The Hartford",
        "sector": "Financials"
    },
    {
        "symbol": "HSY",
        "name": "The Hershey Company",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "MOS",
        "name": "The Mosaic Company",
        "sector": "Materials"
    },
    {
        "symbol": "TRV",
        "name": "The Travelers Companies",
        "sector": "Financials"
    },
    {
        "symbol": "DIS",
        "name": "The Walt Disney Company",
        "sector": "Communication Services"
    },
    {
        "symbol": "TMO",
        "name": "Thermo Fisher Scientific",
        "sector": "Health Care"
    },
    {
        "symbol": "TJX",
        "name": "TJX Companies",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "TSCO",
        "name": "Tractor Supply Company",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "TT",
        "name": "Trane Technologies",
        "sector": "Industrials"
    },
    {
        "symbol": "TDG",
        "name": "TransDigm Group",
        "sector": "Industrials"
    },
    {
        "symbol": "TRMB",
        "name": "Trimble",
        "sector": "Information Technology"
    },
    {
        "symbol": "TFC",
        "name": "Truist Financial",
        "sector": "Financials"
    },
    {
        "symbol": "TWTR",
        "name": "Twitter",
        "sector": "Communication Services"
    },
    {
        "symbol": "TYL",
        "name": "Tyler Technologies",
        "sector": "Information Technology"
    },
    {
        "symbol": "TSN",
        "name": "Tyson Foods",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "USB",
        "name": "U.S. Bancorp",
        "sector": "Financials"
    },
    {
        "symbol": "UDR",
        "name": "UDR",
        "sector": "Real Estate"
    },
    {
        "symbol": "ULTA",
        "name": "Ulta Beauty",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "UAA",
        "name": "Under Armour (Class A)",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "UA",
        "name": "Under Armour (Class C)",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "UNP",
        "name": "Union Pacific",
        "sector": "Industrials"
    },
    {
        "symbol": "UAL",
        "name": "United Airlines",
        "sector": "Industrials"
    },
    {
        "symbol": "UPS",
        "name": "United Parcel Service",
        "sector": "Industrials"
    },
    {
        "symbol": "URI",
        "name": "United Rentals",
        "sector": "Industrials"
    },
    {
        "symbol": "UNH",
        "name": "UnitedHealth Group",
        "sector": "Health Care"
    },
    {
        "symbol": "UHS",
        "name": "Universal Health Services",
        "sector": "Health Care"
    },
    {
        "symbol": "VLO",
        "name": "Valero Energy",
        "sector": "Energy"
    },
    {
        "symbol": "VTR",
        "name": "Ventas",
        "sector": "Real Estate"
    },
    {
        "symbol": "VRSN",
        "name": "Verisign",
        "sector": "Information Technology"
    },
    {
        "symbol": "VRSK",
        "name": "Verisk Analytics",
        "sector": "Industrials"
    },
    {
        "symbol": "VZ",
        "name": "Verizon Communications",
        "sector": "Communication Services"
    },
    {
        "symbol": "VRTX",
        "name": "Vertex Pharmaceuticals",
        "sector": "Health Care"
    },
    {
        "symbol": "VFC",
        "name": "VF Corporation",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "VIAC",
        "name": "ViacomCBS",
        "sector": "Communication Services"
    },
    {
        "symbol": "VTRS",
        "name": "Viatris",
        "sector": "Health Care"
    },
    {
        "symbol": "V",
        "name": "Visa",
        "sector": "Information Technology"
    },
    {
        "symbol": "VNO",
        "name": "Vornado Realty Trust",
        "sector": "Real Estate"
    },
    {
        "symbol": "VMC",
        "name": "Vulcan Materials",
        "sector": "Materials"
    },
    {
        "symbol": "WRB",
        "name": "W. R. Berkley Corporation",
        "sector": "Financials"
    },
    {
        "symbol": "GWW",
        "name": "W. W. Grainger",
        "sector": "Industrials"
    },
    {
        "symbol": "WAB",
        "name": "Wabtec",
        "sector": "Industrials"
    },
    {
        "symbol": "WBA",
        "name": "Walgreens Boots Alliance",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "WMT",
        "name": "Walmart",
        "sector": "Consumer Staples"
    },
    {
        "symbol": "WM",
        "name": "Waste Management",
        "sector": "Industrials"
    },
    {
        "symbol": "WAT",
        "name": "Waters Corporation",
        "sector": "Health Care"
    },
    {
        "symbol": "WEC",
        "name": "WEC Energy Group",
        "sector": "Utilities"
    },
    {
        "symbol": "WFC",
        "name": "Wells Fargo",
        "sector": "Financials"
    },
    {
        "symbol": "WELL",
        "name": "Welltower",
        "sector": "Real Estate"
    },
    {
        "symbol": "WST",
        "name": "West Pharmaceutical Services",
        "sector": "Health Care"
    },
    {
        "symbol": "WDC",
        "name": "Western Digital",
        "sector": "Information Technology"
    },
    {
        "symbol": "WU",
        "name": "Western Union",
        "sector": "Information Technology"
    },
    {
        "symbol": "WRK",
        "name": "WestRock",
        "sector": "Materials"
    },
    {
        "symbol": "WY",
        "name": "Weyerhaeuser",
        "sector": "Real Estate"
    },
    {
        "symbol": "WHR",
        "name": "Whirlpool Corporation",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "WMB",
        "name": "Williams Companies",
        "sector": "Energy"
    },
    {
        "symbol": "WLTW",
        "name": "Willis Towers Watson",
        "sector": "Financials"
    },
    {
        "symbol": "WYNN",
        "name": "Wynn Resorts",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "XEL",
        "name": "Xcel Energy",
        "sector": "Utilities"
    },
    {
        "symbol": "XLNX",
        "name": "Xilinx",
        "sector": "Information Technology"
    },
    {
        "symbol": "XYL",
        "name": "Xylem",
        "sector": "Industrials"
    },
    {
        "symbol": "YUM",
        "name": "Yum! Brands",
        "sector": "Consumer Discretionary"
    },
    {
        "symbol": "ZBRA",
        "name": "Zebra Technologies",
        "sector": "Information Technology"
    },
    {
        "symbol": "ZBH",
        "name": "Zimmer Biomet",
        "sector": "Health Care"
    },
    {
        "symbol": "ZION",
        "name": "Zions Bancorp",
        "sector": "Financials"
    },
    {
        "symbol": "ZTS",
        "name": "Zoetis",
        "sector": "Health Care"
    }
]}"#;

    let json: SPInstruments = serde_json::from_str(&content).expect("JSON was not well-formatted");
    let mut arr = vec![];
    for symbol in json.symbols {
        arr.push([&symbol.symbol, ".US"].concat());
    }

    arr
}

pub fn is_in_sp500(symbol: &str, sp_symbols: &Vec<String>) -> bool {
    let mut result = false;
    for sp_symbol in sp_symbols {
        let s: Vec<&str> = symbol.split("_").collect();
        if s[0] == sp_symbol {
            println!("1111 {:?} {:?}", s[0], sp_symbol);
            result = true;
            break;
        }
    }
    result
}
