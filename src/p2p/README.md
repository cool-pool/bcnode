
NETWORD_ID: 1=BeforeTarget 3=AfterTarget 7=SpectrumBeforeTarget 9=SpectrumAfterTarget 11=Pangaea 

CP_ID: 1 

Status {
  timestamp: 
  greetingKey: a hash of the timestamp and 
  bestHash: 
  bestHashConnectedBlockchain: 
  genesisHash: 
  fingerprintsHash: 
}

Warning {
  Status
  context: The name of the class or function in question 
  reference: id of the data object in question 
}


ObjectTypes = 
  1: block headers
  2: block bodies 
  3: rover best heights 
  4: rover block headers 
  5: rover block bodies 
  6: rover marked headers

getObject { } // request to peer 
getObjects { } // request to peer 

setObject { } // suggestion from peer
setObjects { } // suggestion from peer


Object {
  version: version,
  type: type of data object,
  value: fsdf
}

BlockHeader {
  version: version,
  type: type of data object,
  value: fsdf,
  prevHash: 
  height: 
}
