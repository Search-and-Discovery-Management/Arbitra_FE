const express = require('express')
const app = express()
const port = 3000
var cors = require('cors')

app.use(cors())

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
      name : "Kamisato Ayaka",
      element: "Cryo",
      level: 90,
      attack: 2035,
      defense: 874,
      em : 123,
      nation: "Inazuma",
    },
    {
      weapon: "Amenoma Kageuchi",
      base_attack : 454,
      level : 90,
      substat: "ATK%",
      substat_number: 55.1,
      refinement: 5,
    },
    {
      artifact: "Broken Rime's Echo",
      set_name : "Blizzard Strayer",
      level : 20,
      mainstat: "Crit rate",
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
      name : "Dan Heng",
      element: "Wind",
      path : "The Hunt",
      faction: "Astral Express",
      weapon: "Cloud Piercer",
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
    {
      unit : "Panzer IV",
      type: "Medium Tank",
      armor_front: 247,
      armor_rear: 90,
      damage: 160,
      cost: "340 Manpower, 125 fuel",
    },
    {
      name: "Nobara",
      lastname: "Kugisaki",
      age: 16,
      ability: [
        {
        skill:["Straw Doll","Resonance","Hairpin"],
        weapon:["Hammer","Nails","Rubber Mallet","Straw Doll"],
      },
      ]
    }
    
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