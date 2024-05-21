import * as Cord from '@cord.network/sdk'

/**
 * It creates a document from a schema, content, holder, issuer, authorization and registry
 * @param holder - The DID of the document holder.
 * @param issuer - The DID of the issuer of the document.
 * @param schema - The schema of the document.
 * @param authorization - The authorization id of the issuer.
 * @param registry - The registry where the document is linked.
 * @returns A document
 */
export async function createDocument(
  holder: Cord.DidUri,
  issuer: Cord.DidUri,
  schema: Cord.ISchema,
  chainSpace: Cord.SpaceId,
  signCallback: Cord.SignCallback
): Promise<Cord.IDocument> {
  const content = Cord.Content.fromSchemaAndContent(
    schema,
    {
      name: 'Alice',
      age: 29,
      id: '123456789987654321',
      country: 'India',
      address: {
        street: 'a',
        pin: 54032,
        location: {
          state: 'Karnataka',
          country: 'India',
        },
      },
    },
    holder,
    issuer
  )
  console.log(content, chainSpace)
  const document = Cord.Document.fromContent({
    content,
    chainSpace,
    signCallback,
    options: {},
  })
  return document
}
