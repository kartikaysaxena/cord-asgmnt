import type {
  DidDocument,
  DidKey,
  DidServiceEndpoint,
} from '@cord.network/types'

/**
 * Gets all public keys associated with this DID.
 *
 * @param did The DID data.
 * @returns Array of public keys.
 */
export function getKeys(
  did: Partial<DidDocument> & Pick<DidDocument, 'authentication'>
): DidKey[] {
  return [
    ...did.authentication,
    ...(did.assertionMethod || []),
    ...(did.capabilityDelegation || []),
    ...(did.keyAgreement || []),
  ]
}

/**
 * Returns a key with a given id, if associated with this DID.
 *
 * @param did The DID data.
 * @param id Key id (not the full key uri).
 * @returns The respective public key data or undefined.
 */
export function getKey(
  did: Partial<DidDocument> & Pick<DidDocument, 'authentication'>,
  id: DidKey['id']
): DidKey | undefined {
  return getKeys(did).find((key) => key.id === id)
}

/**
 * Returns a service endpoint with a given id, if associated with this DID.
 *
 * @param did The DID data.
 * @param id Endpoint id (not the full endpoint uri).
 * @returns The respective endpoint data or undefined.
 */
export function getService(
  did: Pick<DidDocument, 'service'>,
  id: DidServiceEndpoint['id']
): DidServiceEndpoint | undefined {
  return did.service?.find((endpoint) => endpoint.id === id)
}
