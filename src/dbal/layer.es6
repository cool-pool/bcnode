/**
 * Copyright (c) 2017-present, blockcollider.org developers, All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @flow
 */

const logging = require('../logger')
const { GetObject, PutObject } = require('../protos/db_pb')
const { inspect } = require('util')

/**
 * Blockchains and their types
 * @type {{bc: *, btc: *, lsk: *}}
 */
export const collections = {
  bc: [],
  btc: [],
  eth: [],
  lsk: [],
  neo: [],
  wav: []
}

/**
 * Database Abstraction Layer
 */
export default class DatabaseLayer {
  _logger: Object // eslint-disable-line no-undef
  _persistence: Object // eslint-disable-line no-undef
  _defaultCollection: string // eslint-disable-line no-undef
  _version: number // eslint-disable-line no-undef
  _collections: Object // eslint-disable-line no-undef

  constructor (persistence: Object) {
    this._logger = logging.getLogger(__filename)
    this._persistence = persistence
    this._defaultCollection = 'bc'
    this._collections = collections
    this._version = 1
  }

  get persistence (): Object {
    return this._persistence
  }

  get collections (): Object {
    return this._collections
  }

  /**
   * Get Ephermeral
   * @param getObject
   */
  getEphermal (query: Object = {}): Promise<*> {
    if (!query) {
      throw Error('no query object provided')
    }
    let q = new GetObject()
    q.setType(query.type)
    q.setKey(query.key)

    if (query.collection !== undefined) {
      q.setCollection(query.collection)
    } else {
      q.setCollection(this._defaultCollection)
    }

    return new Promise((resolve, reject) => {
      const stringQuery =
        q.getCollection() + '.' + q.getType() + '.' + q.getKey()
      this._persistence
        .get(stringQuery)
        .then(result => {
          return resolve(result)
        })
        .catch(e => {
          return reject(e)
        })
    })
  }

  /**
   * Get Object
   * @param getObject
   */
  getObject (query: Object = {}): Promise<*> {
    if (!query) {
      throw Error('no query object provided')
    }
    let q = GetObject
    q.setType(query.type)
    q.setKey(query.key)
    if (query.collection !== undefined) {
      q.setCollection(query.collection)
    } else {
      q.setCollection(this._defaultCollection)
    }

    return new Promise((resolve, reject) => {
      const stringQuery =
        q.getCollection() + '.' + q.getType() + '.' + q.getKey()
      this._persistence
        .get(stringQuery)
        .then(result => {
          return resolve(result)
        })
        .catch(e => {
          return reject(e)
        })
    })
  }

  /**
   * Put Object
   * @param PutObject
   */
  putObject (put: Object = {}): Promise<*> {
    if (!put) {
      throw Error('put object not provided')
    }
    const p = new PutObject()
    p.setType(put.type)
    p.setKey(put.key)
    p.setData(put.value)
    if (put.collection !== undefined) {
      p.setCollection(put.collection)
    } else {
      p.setCollection(this._defaultCollection)
    }
    return new Promise((resolve, reject) => {
      const stringPut =
        p.getCollection() + '.' + p.getType() + '.' + p.getKey()
      console.log(`DatabaseLayer.putObject() - ${inspect(stringPut)}`)
      this._persistence
        .put(stringPut, p.getData())
        .then(result => {
          return resolve(result)
        })
        .catch(e => {
          return reject(e)
        })
    })
  }
}
