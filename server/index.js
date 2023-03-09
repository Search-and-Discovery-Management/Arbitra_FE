const express = require('express')
const app = express()
const port = 3000
var cors = require('cors')
const bodyParser = require('body-parser')

app.use(cors())
app.use(bodyParser.json())
app.use(bodyParser.urlencoded({ extend: true }))

app.get('/', (req, res) => {
  res.send('Hello World!')
})

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})

//GET DATA UNTUK DASHBOARD
app.get('/dashboard_data', (req, res) => {
  var data =
    {
      request_amount: 1412,
      ping: 47,
    }

  let condition = "server error not";

  if (condition == "server error") {
    let error = {
      error_description: "server currently error"
    }
    res.status(500).send(error)
  } else if (condition == "authentication failed") {
    let error = {
      error_description: "auth failed"
    }
    res.status(400).send(error)
  } else {
    res.status(200).send(data)
  }

})

//GET DATA UNTUK STATISTIC DI INDEX PAGE
app.get('/index_stat_data', (req,res) => {
  var data = 
  {
    no_records: 123,
    record_size: 2948,
  }

  let condition = "server error not";

  if (condition == "server error") {
    let error = {
      error_description: "server currently error"
    }
    res.status(500).send(error)
  } else if (condition == "authentication failed") {
    let error = {
      error_description: "auth failed"
    }
    res.status(400).send(error)
  } else {
    res.status(200).send(data)
  }

})

//GET DATA UNTUK CARDS (TEMPORARY!!!)
app.get('/index_card_data', (req,res) => {
  var data = [
    {
      "_id": "JGqSVIYBJ5K9oVSnocgh",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              -22.910461
          ],
          "_geoloc.lng": [
              -43.16313
          ],
          "city": [
              "Rio De Janeiro"
          ],
          "city.keyword": [
              "Rio De Janeiro"
          ],
          "country": [
              "Brazil"
          ],
          "country.keyword": [
              "Brazil"
          ],
          "iata_code": [
              "SDU"
          ],
          "iata_code.keyword": [
              "SDU"
          ],
          "links_count": [
              61
          ],
          "name": [
              "Santos Dumont"
          ],
          "name.keyword": [
              "Santos Dumont"
          ],
          "objectID": [
              "2612"
          ],
          "objectID.keyword": [
              "2612"
          ]
      }
  },
  {
      "_id": "JmqSVIYBJ5K9oVSnochI",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              36.23761
          ],
          "_geoloc.lng": [
              43.963158
          ],
          "city": [
              "Erbil"
          ],
          "city.keyword": [
              "Erbil"
          ],
          "country": [
              "Iraq"
          ],
          "country.keyword": [
              "Iraq"
          ],
          "iata_code": [
              "EBL"
          ],
          "iata_code.keyword": [
              "EBL"
          ],
          "links_count": [
              60
          ],
          "name": [
              "Erbil Intl"
          ],
          "name.keyword": [
              "Erbil Intl"
          ],
          "objectID": [
              "3989"
          ],
          "objectID.keyword": [
              "3989"
          ]
      }
  },
  {
      "_id": "LWqSVIYBJ5K9oVSnociv",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              40.56
          ],
          "_geoloc.lng": [
              109.997
          ],
          "city": [
              "Baotou"
          ],
          "city.keyword": [
              "Baotou"
          ],
          "country": [
              "China"
          ],
          "country.keyword": [
              "China"
          ],
          "iata_code": [
              "BAV"
          ],
          "iata_code.keyword": [
              "BAV"
          ],
          "links_count": [
              58
          ],
          "name": [
              "Baotou Airport"
          ],
          "name.keyword": [
              "Baotou Airport"
          ],
          "objectID": [
              "6346"
          ],
          "objectID.keyword": [
              "6346"
          ]
      }
  },
  {
      "_id": "MGqSVIYBJ5K9oVSnocjV",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              -7.788181
          ],
          "_geoloc.lng": [
              110.431755
          ],
          "city": [
              "Yogyakarta"
          ],
          "city.keyword": [
              "Yogyakarta"
          ],
          "country": [
              "Indonesia"
          ],
          "country.keyword": [
              "Indonesia"
          ],
          "iata_code": [
              "JOG"
          ],
          "iata_code.keyword": [
              "JOG"
          ],
          "links_count": [
              58
          ],
          "name": [
              "Adi Sutjipto"
          ],
          "name.keyword": [
              "Adi Sutjipto"
          ],
          "objectID": [
              "3898"
          ],
          "objectID.keyword": [
              "3898"
          ]
      }
  },
  {
      "_id": "NWqSVIYBJ5K9oVSnosga",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              24.264668
          ],
          "_geoloc.lng": [
              120.62058
          ],
          "city": [
              "Taichung"
          ],
          "city.keyword": [
              "Taichung"
          ],
          "country": [
              "Taiwan"
          ],
          "country.keyword": [
              "Taiwan"
          ],
          "iata_code": [
              "RMQ"
          ],
          "iata_code.keyword": [
              "RMQ"
          ],
          "links_count": [
              58
          ],
          "name": [
              "Ching Chuang Kang"
          ],
          "name.keyword": [
              "Ching Chuang Kang"
          ],
          "objectID": [
              "2268"
          ],
          "objectID.keyword": [
              "2268"
          ]
      }
  },
  {
      "_id": "NmqSVIYBJ5K9oVSnosgl",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              62.462776
          ],
          "_geoloc.lng": [
              -114.44028
          ],
          "city": [
              "Yellowknife"
          ],
          "city.keyword": [
              "Yellowknife"
          ],
          "country": [
              "Canada"
          ],
          "country.keyword": [
              "Canada"
          ],
          "iata_code": [
              "YZF"
          ],
          "iata_code.keyword": [
              "YZF"
          ],
          "links_count": [
              58
          ],
          "name": [
              "Yellowknife"
          ],
          "name.keyword": [
              "Yellowknife"
          ],
          "objectID": [
              "196"
          ],
          "objectID.keyword": [
              "196"
          ]
      }
  },
  {
      "_id": "OmqSVIYBJ5K9oVSnoshV",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              3.543222
          ],
          "_geoloc.lng": [
              -76.381584
          ],
          "city": [
              "Cali"
          ],
          "city.keyword": [
              "Cali"
          ],
          "country": [
              "Colombia"
          ],
          "country.keyword": [
              "Colombia"
          ],
          "iata_code": [
              "CLO"
          ],
          "iata_code.keyword": [
              "CLO"
          ],
          "links_count": [
              57
          ],
          "name": [
              "Alfonso Bonilla Aragon Intl"
          ],
          "name.keyword": [
              "Alfonso Bonilla Aragon Intl"
          ],
          "objectID": [
              "2715"
          ],
          "objectID.keyword": [
              "2715"
          ]
      }
  },
  {
      "_id": "O2qSVIYBJ5K9oVSnoshh",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              46.79111
          ],
          "_geoloc.lng": [
              -71.39333
          ],
          "city": [
              "Quebec"
          ],
          "city.keyword": [
              "Quebec"
          ],
          "country": [
              "Canada"
          ],
          "country.keyword": [
              "Canada"
          ],
          "iata_code": [
              "YQB"
          ],
          "iata_code.keyword": [
              "YQB"
          ],
          "links_count": [
              57
          ],
          "name": [
              "Quebec Jean Lesage Intl"
          ],
          "name.keyword": [
              "Quebec Jean Lesage Intl"
          ],
          "objectID": [
              "111"
          ],
          "objectID.keyword": [
              "111"
          ]
      }
  },
  {
      "_id": "PmqSVIYBJ5K9oVSnosiN",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              39.499107
          ],
          "_geoloc.lng": [
              -119.768105
          ],
          "city": [
              "Reno"
          ],
          "city.keyword": [
              "Reno"
          ],
          "country": [
              "United States"
          ],
          "country.keyword": [
              "United States"
          ],
          "iata_code": [
              "RNO"
          ],
          "iata_code.keyword": [
              "RNO"
          ],
          "links_count": [
              56
          ],
          "name": [
              "Reno Tahoe Intl"
          ],
          "name.keyword": [
              "Reno Tahoe Intl"
          ],
          "objectID": [
              "3807"
          ],
          "objectID.keyword": [
              "3807"
          ]
      }
  },
  {
      "_id": "P2qSVIYBJ5K9oVSnosiY",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              42.880833
          ],
          "_geoloc.lng": [
              -85.522804
          ],
          "city": [
              "Grand Rapids"
          ],
          "city.keyword": [
              "Grand Rapids"
          ],
          "country": [
              "United States"
          ],
          "country.keyword": [
              "United States"
          ],
          "iata_code": [
              "GRR"
          ],
          "iata_code.keyword": [
              "GRR"
          ],
          "links_count": [
              56
          ],
          "name": [
              "Gerald R Ford Intl"
          ],
          "name.keyword": [
              "Gerald R Ford Intl"
          ],
          "objectID": [
              "3685"
          ],
          "objectID.keyword": [
              "3685"
          ]
      }
  },
  {
      "_id": "QGqSVIYBJ5K9oVSnosim",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              -28.164444
          ],
          "_geoloc.lng": [
              153.50471
          ],
          "city": [
              "Coolangatta"
          ],
          "city.keyword": [
              "Coolangatta"
          ],
          "country": [
              "Australia"
          ],
          "country.keyword": [
              "Australia"
          ],
          "iata_code": [
              "OOL"
          ],
          "iata_code.keyword": [
              "OOL"
          ],
          "links_count": [
              56
          ],
          "name": [
              "Gold Coast"
          ],
          "name.keyword": [
              "Gold Coast"
          ],
          "objectID": [
              "3321"
          ],
          "objectID.keyword": [
              "3321"
          ]
      }
  },
  {
      "_id": "QWqSVIYBJ5K9oVSnosiw",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              55.606186
          ],
          "_geoloc.lng": [
              49.27873
          ],
          "city": [
              "Kazan"
          ],
          "city.keyword": [
              "Kazan"
          ],
          "country": [
              "Russia"
          ],
          "country.keyword": [
              "Russia"
          ],
          "iata_code": [
              "KZN"
          ],
          "iata_code.keyword": [
              "KZN"
          ],
          "links_count": [
              56
          ],
          "name": [
              "Kazan"
          ],
          "name.keyword": [
              "Kazan"
          ],
          "objectID": [
              "2990"
          ],
          "objectID.keyword": [
              "2990"
          ]
      }
  },
  {
      "_id": "RGqSVIYBJ5K9oVSnosjb",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              17.539145
          ],
          "_geoloc.lng": [
              -88.308205
          ],
          "city": [
              "Belize City"
          ],
          "city.keyword": [
              "Belize City"
          ],
          "country": [
              "Belize"
          ],
          "country.keyword": [
              "Belize"
          ],
          "iata_code": [
              "BZE"
          ],
          "iata_code.keyword": [
              "BZE"
          ],
          "links_count": [
              56
          ],
          "name": [
              "Philip S W Goldson Intl"
          ],
          "name.keyword": [
              "Philip S W Goldson Intl"
          ],
          "objectID": [
              "1957"
          ],
          "objectID.keyword": [
              "1957"
          ]
      }
  },
  {
      "_id": "RWqSVIYBJ5K9oVSnosjs",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              37.911404
          ],
          "_geoloc.lng": [
              12.487961
          ],
          "city": [
              "Trapani"
          ],
          "city.keyword": [
              "Trapani"
          ],
          "country": [
              "Italy"
          ],
          "country.keyword": [
              "Italy"
          ],
          "iata_code": [
              "TPS"
          ],
          "iata_code.keyword": [
              "TPS"
          ],
          "links_count": [
              56
          ],
          "name": [
              "Trapani Birgi"
          ],
          "name.keyword": [
              "Trapani Birgi"
          ],
          "objectID": [
              "1515"
          ],
          "objectID.keyword": [
              "1515"
          ]
      }
  },
  {
      "_id": "SGqSVIYBJ5K9oVSno8gi",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              41.732582
          ],
          "_geoloc.lng": [
              -71.42038
          ],
          "city": [
              "Providence"
          ],
          "city.keyword": [
              "Providence"
          ],
          "country": [
              "United States"
          ],
          "country.keyword": [
              "United States"
          ],
          "iata_code": [
              "PVD"
          ],
          "iata_code.keyword": [
              "PVD"
          ],
          "links_count": [
              55
          ],
          "name": [
              "Theodore Francis Green State"
          ],
          "name.keyword": [
              "Theodore Francis Green State"
          ],
          "objectID": [
              "3641"
          ],
          "objectID.keyword": [
              "3641"
          ]
      }
  },
  {
      "_id": "TGqSVIYBJ5K9oVSno8h5",
      "_index": "airplanes_v3",
      "_score": 1.0,
      "fields": {
          "_geoloc.lat": [
              59.35437
          ],
          "_geoloc.lng": [
              17.94165
          ],
          "city": [
              "Stockholm"
          ],
          "city.keyword": [
              "Stockholm"
          ],
          "country": [
              "Sweden"
          ],
          "country.keyword": [
              "Sweden"
          ],
          "iata_code": [
              "BMA"
          ],
          "iata_code.keyword": [
              "BMA"
          ],
          "links_count": [
              54
          ],
          "name": [
              "Bromma"
          ],
          "name.keyword": [
              "Bromma"
          ],
          "objectID": [
              "738"
          ],
          "objectID.keyword": [
              "738"
          ]
      }
  },
    
  ];
  
  let condition = "server error not";

  if (condition == "server error") {
    let error = {
      error_description: "server currently error"
    }
    res.status(500).send(error)
  } else if (condition == "authentication failed") {
    let error = {
      error_description: "auth failed"
    }
    res.status(400).send(error)
  } else {
    res.status(200).send(data)
  }


  
})

//Testing
app.post('/attack', (req, res) => {

  console.log(req.body)
  var data = [
    {
      name : "Kamisato Ayaka",
      element: "Cryo",
      level: 90,
      attack: 2035,
      defense: 874,
      em : 123,
      nation: "Inazuma",
    },
    {
      name : "Xingqiu",
      element: "Hydro",
      level: 80,
      attack: 1345,
      defense: 763,
      em : 255,
      nation: "Liyue",  
    },
    {
      name : "Yelan",
      element: "Hydro",
      level: 90,
      attack: 2042,
      defense: 980,
      em : 203,
      nation: "Liyue",  
    },
    {
      name : "Nahida",
      element: "Dendro",
      level: 80,
      attack: 1603,
      defense: 587,
      em : 954,
      nation: "Sumeru",  
    },
    
  ];

  let condition = "server error not";

  if (condition == "server error") {
    let error = {
      error_description: "server currently error"
    }
    res.status(500).send(error)
  } else if (condition == "authentication failed") {
    let error = {
      error_description: "auth failed"
    }
    res.status(400).send(error)
  } else {
    res.status(200).send(data)
  }

})