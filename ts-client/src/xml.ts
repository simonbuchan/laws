import { parseXml, XmlElement, XmlNode, XmlText } from "@rgrove/parse-xml";

/**
 * Formats input like:
 *
 * ```json
 * {
 *   Foo: "bar",
 *   Baz: ["qux", "quux"],
 *   Corge: {
 *     Grault: "garply",
 *   }
 * }
 * ```
 *
 * Into:
 *
 * ```xml
 * <operation.name xmlns={operation.namespace}">
 *   <Foo>bar</Foo>
 *   <Baz>
 *     <member>qux</member>
 *     <member>quux</member>
 *   </Baz>
 *   <Corge>
 *     <Grault>garply</Grault>
 *   </Corge>
 * </operation.name>
 * ```
 */
export function formatRequest(
  json: unknown,
  name: string,
  namespace: string,
  prefix: string | undefined,
): string {
  const root = new XmlElement(name, {
    [prefix ? `xmlns:${prefix}` : "xmlns"]: namespace,
  });
  formatObject(json, root);
  return root.toString();
}

function formatObject(json: unknown, parent: XmlElement) {
  if (typeof json === "string") {
    parent.children.push(new XmlText(json));
  } else if (Array.isArray(json)) {
    for (const item of json) {
      formatObject(item, parent);
    }
  } else if (typeof json === "object") {
    for (const [key, value] of Object.entries(
      json as Record<string, unknown>,
    )) {
      if (value !== undefined) {
        const child = new XmlElement(key);
        parent.children.push(child);
        formatObject(value, child);
      }
    }
  }
}

/**
 * Parses input like:
 *
 * ```xml
 * <FooResponse>
 *   <FooResult>...</FooResult>
 *   <ResponseMetadata></ResponseMetadata>
 * </FooResponse>
 * ```
 *
 * Into an object with the Result element as the root and ResponseMetadata
 */
export function parseResponse(
  source: string,
  name: string,
  namespace: string,
  prefix: string | undefined,
): unknown {
  const document = parseXml(source);
  const root = document.root;
  if (!root) {
    throw new Error("Missing root element");
  }
  const result = root.children.find(
    (child): child is XmlElement =>
      child instanceof XmlElement && child.name === name,
  );
  const metadata = root.children.find(
    (child): child is XmlElement =>
      child instanceof XmlElement && child.name === "ResponseMetadata",
  );

  return {
    ...xmlNodeToJson(result),
    $metadata: xmlNodeToJson(metadata),
  };
}

function xmlNodeToJson(node: XmlElement): Record<string, unknown>;
function xmlNodeToJson(node: XmlText): string;
function xmlNodeToJson(node: undefined): undefined;
function xmlNodeToJson(
  node: XmlElement | undefined,
): Record<string, unknown> | undefined;
function xmlNodeToJson(node: XmlText | undefined): string | undefined;
function xmlNodeToJson(node: XmlNode | undefined): unknown {
  if (!node) {
    return undefined;
  }
  if (node instanceof XmlElement) {
    if (node.children.length === 1 && node.children[0] instanceof XmlText) {
      return node.children[0].text;
    }
    const json: Record<string, unknown> = {};
    for (const child of node.children) {
      if (child instanceof XmlElement) {
        const key = child.name;
        const value = xmlNodeToJson(child);
        if (key in json) {
          if (!Array.isArray(json[key])) {
            json[key] = [json[key] as unknown];
          }
          (json[key] as unknown[]).push(value);
        } else {
          json[key] = value;
        }
      }
    }
    return json;
  }
  if (node instanceof XmlText) {
    return node.text;
  }
  throw new Error(`Unexpected node type: ${node.type}`);
}
