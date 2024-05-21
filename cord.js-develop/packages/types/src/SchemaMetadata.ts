import type { ISchema } from './Schema'

/**
 * String struct with string keys and a mandatory `default` field.
 * Meant to contain a default label/description and an arbitrary number of translations,
 * where keys represent the use case (language) and values are the labels for this use case.
 */
export interface IMultilangLabel {
  /** Default label in the original language. */
  default: string
  /** An arbitrary number of translations where the key indicates the language. */
  [key: string]: string
}

export type IMetadataProperties = {
  [key: string]: { title: IMultilangLabel; description?: IMultilangLabel }
}

export interface IMetadata {
  title: IMultilangLabel
  description?: IMultilangLabel
  properties: IMetadataProperties
}

export interface ISchemaMetadata {
  metadata: IMetadata
  id: ISchema['$id'] | null
}
