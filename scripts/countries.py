#!/usr/bin/env python3
#
# Helper script that transforms countries in various non-standard forms
# into the full names used by modules/opltypes/src/countries.rs.
#
# When run as a script, it fixes the "Country" columns.
#
# This is also set up to be importable as a library, so that importation
# scripts can use it directly.

import oplcsv
import sys
import os


# Map of substitutions.
COUNTRY_MAP = {
    # International Olympic Committee country codes.
    # Also some common typos of those codes.
    'AFG': 'Afghanistan',
    'ALB': 'Albania',
    'ALG': 'Algeria',
    'AHO': 'Netherlands Antilles',
    'AND': 'Andorra',
    'ANG': 'Angola',
    'ANT': 'Antigua and Barbuda',
    'ARG': 'Argentina',
    'ARM': 'Armenia',
    'ARU': 'Aruba',
    'ASA': 'American Samoa',
    'AUS': 'Australia',
    'AUT': 'Austria',
    'AZE': 'Azerbaijan',
    'BAH': 'Bahamas',
    'BAN': 'Bangladesh',
    'BAR': 'Barbados',
    'BDI': 'Burundi',
    'BEL': 'Belgium',
    'BEN': 'Benin',
    'BER': 'Bermuda',
    'BHU': 'Bhutan',
    'BIH': 'Bosnia and Herzegovina',
    'BIZ': 'Belize',
    'BLR': 'Belarus',
    'BOL': 'Bolivia',
    'BOT': 'Botswana',
    'BRA': 'Brazil',
    'BRN': 'Bahrain',
    'BRU': 'Brunei',
    'BUL': 'Bulgaria',
    'BUR': 'Burkina Faso',
    'CAF': 'Central African Republic',
    'CAM': 'Cambodia',
    'CAN': 'Canada',
    'CAY': 'Cayman Islands',
    'CGO': 'Congo',
    'CHA': 'Chad',
    'CHI': 'Chile',
    'CHN': 'China',
    'CIV': 'Côte d’Ivoire',
    'IVC': 'Côte d’Ivoire',
    'CMR': 'Cameroon',
    'COD': 'Democratic Republic of the Congo',
    'COK': 'Cook Islands',
    'COL': 'Colombia',
    'COM': 'Comoros',
    'CPV': 'Cape Verde',
    'CRC': 'Costa Rica',
    'CRO': 'Croatia',
    'CUB': 'Cuba',
    'CYP': 'Cyprus',
    'CZE': 'Czechia',
    'CZR': 'Czechia',
    'TCH': 'Czechia',
    'DAN': 'Denmark',
    'DEN': 'Denmark',
    'DNK': 'Denmark',
    'DJI': 'Djibouti',
    'DMA': 'Dominica',
    'DOM': 'Dominican Republic',
    'ECU': 'Ecuador',
    'EGY': 'Egypt',
    'ERI': 'Eritrea',
    'ESA': 'El Salvador',
    'ESP': 'Spain',
    'SPA': 'Spain',
    'EST': 'Estonia',
    'ETH': 'Ethiopia',
    'FIJ': 'Fiji',
    'FIN': 'Finland',
    'FRA': 'France',
    'FRG': 'West Germany',
    'FGR': 'West Germany',
    'FSM': 'Federated States of Micronesia',
    'GAB': 'Gabon',
    'GAM': 'The Gambia',
    'GBR': 'Britain',
    'GBS': 'Guinea-Bissau',
    'GEO': 'Georgia',
    'ENG': 'England',
    'GEQ': 'Equatorial Guinea',
    'GER': 'Germany',
    'GHA': 'Ghana',
    'GRE': 'Greece',
    'GRN': 'Grenada',
    'GUA': 'Guatemala',
    'GUI': 'Guinea',
    'GUM': 'Guam',
    'GUY': 'Guyana',
    'HAI': 'Haiti',
    'HKG': 'Hong Kong',
    'HON': 'Honduras',
    'HUN': 'Hungary',
    'INA': 'Indonesia',
    'IND': 'India',
    'IRI': 'Iran',
    'IRN': 'Iran',
    'IRE': 'Ireland',
    'IRL': 'Ireland',
    'IRQ': 'Iraq',
    'ISL': 'Iceland',
    'ICE': 'Iceland',
    'ICL': 'Iceland',
    'ISR': 'Israel',
    'ISV': 'US Virgin Islands',
    'ITA': 'Italy',
    'IVB': 'British Virgin Islands',
    'JAM': 'Jamaica',
    'JOR': 'Jordan',
    'JPN': 'Japan',
    'JAP': 'Japan',
    'KAZ': 'Kazakhstan',
    'KEN': 'Kenya',
    'KGZ': 'Kyrgyzstan',
    'KIR': 'Kiribati',
    'KOR': 'South Korea',
    'KOS': 'Kosovo',
    'KSA': 'Saudi Arabia',
    'KUW': 'Kuwait',
    'LAO': 'Laos',
    'LAT': 'Latvia',
    'LBA': 'Libya',
    'LYB': 'Libya',
    'LBN': 'Lebanon',
    'LIB': 'Lebanon',
    'LBR': 'Liberia',
    'LCA': 'Saint Lucia',
    'LES': 'Lesotho',
    'LIE': 'Liechtenstein',
    'LTU': 'Lithuania',
    'LIT': 'Lithuania',
    'LTH': 'Lithuania',
    'LUX': 'Luxembourg',
    'MAD': 'Madagascar',
    'MAR': 'Morocco',
    'MAS': 'Malaysia',
    'MAW': 'Malawi',
    'MDA': 'Moldova',
    'MDV': 'Maldives',
    'MEX': 'Mexico',
    'MGL': 'Mongolia',
    'MHL': 'Marshall Islands',
    'MKD': 'Macedonia',
    'MLI': 'Mali',
    'MLT': 'Malta',
    'MNE': 'Montenegro',
    'MON': 'Monaco',
    'MOZ': 'Mozambique',
    'MRI': 'Mauritius',
    'MTN': 'Mauritania',
    'MYA': 'Myanmar',
    'NAM': 'Namibia',
    'NCA': 'Nicaragua',
    'NED': 'Netherlands',
    'NET': 'Netherlands',
    'NLD': 'Netherlands',
    'NTH': 'Netherlands',
    'HOL': 'Netherlands',
    'NEP': 'Nepal',
    'NGR': 'Nigeria',
    'NIG': 'Niger',
    'NOR': 'Norway',
    'NRU': 'Nauru',
    'NZL': 'New Zealand',
    'OMA': 'Oman',
    'OMN': 'Oman',
    'PAK': 'Pakistan',
    'PAN': 'Panama',
    'PAR': 'Paraguay',
    'PER': 'Peru',
    'PHI': 'Philippines',
    'PLE': 'Palestine',
    'PLW': 'Palau',
    'PNG': 'Papua New Guinea',
    'POL': 'Poland',
    'POR': 'Portugal',
    'PRK': 'North Korea',
    'PUR': 'Puerto Rico',
    'QAT': 'Qatar',
    'ROU': 'Romania',
    'ROM': 'Romania',
    'RSA': 'South Africa',
    'RUS': 'Russia',
    'RWA': 'Rwanda',
    'SAM': 'Samoa',
    'SEN': 'Senegal',
    'SEY': 'Seychelles',
    'SGP': 'Singapore',
    'SIN': 'Singapore',
    'SKN': 'Saint Kitts and Nevis  ',
    'SLE': 'Sierra Leone',
    'SLO': 'Slovenia',
    'SMR': 'San Marino',
    'SOL': 'Solomon Islands',
    'SOM': 'Somalia',
    'SRB': 'Serbia',
    'SCG': 'Serbia and Montenegro',
    'SRI': 'Sri Lanka',
    'SSD': 'South Sudan',
    'SOV': 'USSR',
    'URS': 'USSR',
    'STP': 'São Tomé and Príncipe',
    'SUD': 'Sudan',
    'SUI': 'Switzerland',
    'SUR': 'Suriname',
    'SVK': 'Slovakia',
    'SWE': 'Sweden',
    'SVE': 'Sweden',
    'SWZ': 'Swaziland',
    'SYR': 'Syria',
    'TAN': 'Tanzania',
    'TGA': 'Tonga',
    'THA': 'Thailand',
    'TJK': 'Tajikistan',
    'TKM': 'Turkmenistan',
    'TLS': 'East',
    'TOG': 'Togo',
    'TPE': 'Taiwan',
    'TAI': 'Taiwan',
    'TTO': 'Trinidad and Tobago',
    'TRI': 'Trinidad and Tobago',
    'TUN': 'Tunisia',
    'TUR': 'Turkey',
    'TUV': 'Tuvalu',
    'UGA': 'Uganda',
    'UKR': 'Ukraine',
    'URU': 'Uruguay',
    'USA': 'USA',
    'UZB': 'Uzbekistan',
    'VAN': 'Vanuatu',
    'VEN': 'Venezuela',
    'VIE': 'Vietnam',
    'VIN': 'Saint Vincent and the Grenadines',
    'YEM': 'Yemen',
    'YUG': 'Yugoslavia',
    'ZAM': 'Zambia',
    'ZIM': 'Zimbabwe',

    # Common substitutions to match our format.
    'Belorussia': 'Belarus',
    'Can': 'Canada',
    'Chinese Taipei': 'Taiwan',
    'C.Taipei': 'Taiwan',
    'C. Taipei': 'Taiwan',
    'Chinese Tai.': 'Taiwan',
    'Chin.Taipei': 'Taiwan',
    'Czech Republic': 'Czechia',
    'Danmark': 'Denmark',
    'Dannmark': 'Denmark',
    'Eng': 'England',
    'Fra': 'France',
    'Great Britain': 'UK',
    'G. Brtain': 'UK',
    'G. Britain': 'UK',
    'Holland': 'Netherlands',
    'Ísland': 'Iceland',
    'Island': 'Iceland',
    'Jap': 'Japan',
    'Kazahkstan': 'Kazakhstan',
    'Kazakstan': 'Kazakhstan',
    'Luxembg': 'Luxembourg',
    'New Zeeland': 'New Zealand',
    'Norge': 'Norway',
    'Norwegen': 'Norway',
    'R.S.Afrika': 'South Africa',
    'R.South Africa': 'South Africa',
    'South-Africa': 'South Africa',
    'South Afrika': 'South Africa',
    'Soviet Union': 'USSR',
    'Sverige': 'Sweden',
    'Trinidad&Tobago': 'Trinidad and Tobago',
    'Ukraina': 'Ukraine',
    'United Arab Emirates': 'UAE',
    'Un.Emirates': 'UAE',
    'US.America': 'USA',
    'U.S..America': 'USA',
    'U.S. America': 'USA',
    'US. America': 'USA',
    'US America': 'USA',
    'U.S.America': 'USA',
    'US': 'USA',
    'Whiterussia': 'Belarus'
}


def standardize_country_csv(csv):
    '''Standardizes the Country column.
       Returns true iff something was changed.'''
    global COUNTRY_MAP

    if 'Country' not in csv.fieldnames:
        return False
    idx = csv.index('Country')

    changed = False
    for row in csv.rows:
        country = row[idx]
        if country in COUNTRY_MAP:
            row[idx] = COUNTRY_MAP[country]
            changed = True

    return changed


def standardize_country_filename(filename):
    csv = oplcsv.Csv(filename)
    if standardize_country_csv(csv):
        csv.write_filename(filename)


if __name__ == '__main__':
    if len(sys.argv) > 1:
        standardize_country_filename(sys.argv[1])
    else:
        for dirname, subdirs, files in os.walk(os.getcwd()):
            if "meet-data" in subdirs:
                subdirs[:] = ['meet-data']
            if 'entries.csv' in files:
                filepath = dirname + os.sep + 'entries.csv'
                standardize_country_filename(filepath)
