/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::types::*;
use dom::bindings::codegen::*;
use dom::bindings::utils::{BindingObject, WrapperCache, CacheableWrapper};
use dom::node::ScriptView;

use js::jsapi::{JSContext, JSObject};

pub macro_rules! generate_cacheable_wrapper(
    ($name: path, $wrap: path) => (
        impl CacheableWrapper for $name {
            fn get_wrappercache(&mut self) -> &mut WrapperCache {
                self.parent.get_wrappercache()
            }

            fn wrap_object_shared(@mut self, cx: *JSContext, scope: *JSObject) -> *JSObject {
                let mut unused = false;
                $wrap(cx, scope, self, &mut unused)
            }
        }
    )
)

pub macro_rules! generate_binding_object(
    ($name: path) => (
        impl BindingObject for $name {
            fn GetParentObject(&self, cx: *JSContext) -> Option<@mut CacheableWrapper> {
                self.parent.GetParentObject(cx)
            }
        }
    )
)

generate_cacheable_wrapper!(Comment, CommentBinding::Wrap)
generate_binding_object!(Comment)
generate_cacheable_wrapper!(DocumentType<ScriptView>, DocumentTypeBinding::Wrap)
generate_binding_object!(DocumentType<ScriptView>)
generate_cacheable_wrapper!(Text, TextBinding::Wrap)
generate_binding_object!(Text)
generate_cacheable_wrapper!(HTMLHeadElement, HTMLHeadElementBinding::Wrap)
generate_binding_object!(HTMLHeadElement)
generate_cacheable_wrapper!(HTMLAnchorElement, HTMLAnchorElementBinding::Wrap)
generate_binding_object!(HTMLAnchorElement)
generate_cacheable_wrapper!(HTMLAppletElement, HTMLAppletElementBinding::Wrap)
generate_binding_object!(HTMLAppletElement)
generate_cacheable_wrapper!(HTMLAreaElement, HTMLAreaElementBinding::Wrap)
generate_binding_object!(HTMLAreaElement)
generate_cacheable_wrapper!(HTMLBaseElement, HTMLBaseElementBinding::Wrap)
generate_binding_object!(HTMLBaseElement)
generate_cacheable_wrapper!(HTMLBodyElement, HTMLBodyElementBinding::Wrap)
generate_binding_object!(HTMLBodyElement)
generate_cacheable_wrapper!(HTMLButtonElement, HTMLButtonElementBinding::Wrap)
generate_binding_object!(HTMLButtonElement)
generate_cacheable_wrapper!(HTMLCanvasElement, HTMLCanvasElementBinding::Wrap)
generate_binding_object!(HTMLCanvasElement)
generate_cacheable_wrapper!(HTMLDataListElement, HTMLDataListElementBinding::Wrap)
generate_binding_object!(HTMLDataListElement)
generate_cacheable_wrapper!(HTMLDListElement, HTMLDListElementBinding::Wrap)
generate_binding_object!(HTMLDListElement)
generate_cacheable_wrapper!(HTMLFormElement, HTMLFormElementBinding::Wrap)
generate_binding_object!(HTMLFormElement)
generate_cacheable_wrapper!(HTMLFrameElement, HTMLFrameElementBinding::Wrap)
generate_binding_object!(HTMLFrameElement)
generate_cacheable_wrapper!(HTMLFrameSetElement, HTMLFrameSetElementBinding::Wrap)
generate_binding_object!(HTMLFrameSetElement)
generate_cacheable_wrapper!(HTMLBRElement, HTMLBRElementBinding::Wrap)
generate_binding_object!(HTMLBRElement)
generate_cacheable_wrapper!(HTMLHRElement, HTMLHRElementBinding::Wrap)
generate_binding_object!(HTMLHRElement)
generate_cacheable_wrapper!(HTMLHtmlElement, HTMLHtmlElementBinding::Wrap)
generate_binding_object!(HTMLHtmlElement)
generate_cacheable_wrapper!(HTMLDataElement, HTMLDataElementBinding::Wrap)
generate_binding_object!(HTMLDataElement)
generate_cacheable_wrapper!(HTMLDirectoryElement, HTMLDirectoryElementBinding::Wrap)
generate_binding_object!(HTMLDirectoryElement)
generate_cacheable_wrapper!(HTMLDivElement, HTMLDivElementBinding::Wrap)
generate_binding_object!(HTMLDivElement)
generate_cacheable_wrapper!(HTMLEmbedElement, HTMLEmbedElementBinding::Wrap)
generate_binding_object!(HTMLEmbedElement)
generate_cacheable_wrapper!(HTMLFieldSetElement, HTMLFieldSetElementBinding::Wrap)
generate_binding_object!(HTMLFieldSetElement)
generate_cacheable_wrapper!(HTMLFontElement, HTMLFontElementBinding::Wrap)
generate_binding_object!(HTMLFontElement)
generate_cacheable_wrapper!(HTMLHeadingElement, HTMLHeadingElementBinding::Wrap)
generate_binding_object!(HTMLHeadingElement)
generate_cacheable_wrapper!(HTMLIFrameElement, HTMLIFrameElementBinding::Wrap)
generate_binding_object!(HTMLIFrameElement)
generate_cacheable_wrapper!(HTMLImageElement, HTMLImageElementBinding::Wrap)
generate_binding_object!(HTMLImageElement)
generate_cacheable_wrapper!(HTMLInputElement, HTMLInputElementBinding::Wrap)
generate_binding_object!(HTMLInputElement)
generate_cacheable_wrapper!(HTMLLIElement, HTMLLIElementBinding::Wrap)
generate_binding_object!(HTMLLIElement)
generate_cacheable_wrapper!(HTMLLinkElement, HTMLLinkElementBinding::Wrap)
generate_binding_object!(HTMLLinkElement)
generate_cacheable_wrapper!(HTMLMapElement, HTMLMapElementBinding::Wrap)
generate_binding_object!(HTMLMapElement)
generate_cacheable_wrapper!(HTMLMetaElement, HTMLMetaElementBinding::Wrap)
generate_binding_object!(HTMLMetaElement)
generate_cacheable_wrapper!(HTMLMeterElement, HTMLMeterElementBinding::Wrap)
generate_binding_object!(HTMLMeterElement)
generate_cacheable_wrapper!(HTMLModElement, HTMLModElementBinding::Wrap)
generate_binding_object!(HTMLModElement)
generate_cacheable_wrapper!(HTMLObjectElement, HTMLObjectElementBinding::Wrap)
generate_binding_object!(HTMLObjectElement)
generate_cacheable_wrapper!(HTMLOListElement, HTMLOListElementBinding::Wrap)
generate_binding_object!(HTMLOListElement)
generate_cacheable_wrapper!(HTMLOptGroupElement, HTMLOptGroupElementBinding::Wrap)
generate_binding_object!(HTMLOptGroupElement)
generate_cacheable_wrapper!(HTMLOptionElement, HTMLOptionElementBinding::Wrap)
generate_binding_object!(HTMLOptionElement)
generate_cacheable_wrapper!(HTMLOutputElement, HTMLOutputElementBinding::Wrap)
generate_binding_object!(HTMLOutputElement)
generate_cacheable_wrapper!(HTMLParagraphElement, HTMLParagraphElementBinding::Wrap)
generate_binding_object!(HTMLParagraphElement)
generate_cacheable_wrapper!(HTMLParamElement, HTMLParamElementBinding::Wrap)
generate_binding_object!(HTMLParamElement)
generate_cacheable_wrapper!(HTMLProgressElement, HTMLProgressElementBinding::Wrap)
generate_binding_object!(HTMLProgressElement)
generate_cacheable_wrapper!(HTMLQuoteElement, HTMLQuoteElementBinding::Wrap)
generate_binding_object!(HTMLQuoteElement)
generate_cacheable_wrapper!(HTMLScriptElement, HTMLScriptElementBinding::Wrap)
generate_binding_object!(HTMLScriptElement)
generate_cacheable_wrapper!(HTMLSelectElement, HTMLSelectElementBinding::Wrap)
generate_binding_object!(HTMLSelectElement)
generate_cacheable_wrapper!(HTMLSourceElement, HTMLSourceElementBinding::Wrap)
generate_binding_object!(HTMLSourceElement)
generate_cacheable_wrapper!(HTMLSpanElement, HTMLSpanElementBinding::Wrap)
generate_binding_object!(HTMLSpanElement)
generate_cacheable_wrapper!(HTMLStyleElement, HTMLStyleElementBinding::Wrap)
generate_binding_object!(HTMLStyleElement)
generate_cacheable_wrapper!(HTMLTableElement, HTMLTableElementBinding::Wrap)
generate_binding_object!(HTMLTableElement)
generate_cacheable_wrapper!(HTMLTableCaptionElement, HTMLTableCaptionElementBinding::Wrap)
generate_binding_object!(HTMLTableCaptionElement)
generate_cacheable_wrapper!(HTMLTableCellElement, HTMLTableCellElementBinding::Wrap)
generate_binding_object!(HTMLTableCellElement)
generate_cacheable_wrapper!(HTMLTableColElement, HTMLTableColElementBinding::Wrap)
generate_binding_object!(HTMLTableColElement)
generate_cacheable_wrapper!(HTMLTableRowElement, HTMLTableRowElementBinding::Wrap)
generate_binding_object!(HTMLTableRowElement)
generate_cacheable_wrapper!(HTMLTableSectionElement, HTMLTableSectionElementBinding::Wrap)
generate_binding_object!(HTMLTableSectionElement)
generate_cacheable_wrapper!(HTMLTextAreaElement, HTMLTextAreaElementBinding::Wrap)
generate_binding_object!(HTMLTextAreaElement)
generate_cacheable_wrapper!(HTMLTitleElement, HTMLTitleElementBinding::Wrap)
generate_binding_object!(HTMLTitleElement)
generate_cacheable_wrapper!(HTMLTimeElement, HTMLTimeElementBinding::Wrap)
generate_binding_object!(HTMLTimeElement)
generate_cacheable_wrapper!(HTMLUListElement, HTMLUListElementBinding::Wrap)
generate_binding_object!(HTMLUListElement)
generate_cacheable_wrapper!(HTMLUnknownElement, HTMLUnknownElementBinding::Wrap)
generate_binding_object!(HTMLUnknownElement)
