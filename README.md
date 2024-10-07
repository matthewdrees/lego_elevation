# lego_elevation

Get elevation data suitable for building a terrain map out of legos.

# Usage

    Usage: lego_elevation [OPTIONS] --center <CENTER> --radius <RADIUS> --levels <LEVELS> --gridsize <GRIDSIZE>

    Options:
      -c, --center <CENTER>      Center of the map, latitude/longitude. Supported formats:
                                     "46° 51' 6 N 121° 45' 37 W"
                                     "N 46° 51' 6, W 121° 45' 37"
                                     "46° 51.1' N 121° 58.6167' W"
                                     "46.86167° N, 121.76028° W"
                                     "46.86167 N 121.76028 W"
      -r, --radius <RADIUS>      Map radius from the center, in kilometers
      -l, --levels <LEVELS>      Number of elevation levels
      -g, --gridsize <GRIDSIZE>  Number of columns and rows
      -v, --verbose
      -h, --help                 Print help (see more with '--help')
      -V, --version              Print version

    Example: (Mount Rainier)

      $ lego_elevation --center "46°51′6 N 121°45′37 W" --radius 7 --levels 9 --gridsize 32

    Elevation values are written to 'elevation.csv'.

# Limitations

  * Currently uses the USGS Single Point Query service so fetching data is slow.
  * Elevation data only available on land in Canada, Mexico, and USA. (TODO: use to the [ETOPO Global Relief Model](https://www.ncei.noaa.gov/products/etopo-global-relief-model)).
