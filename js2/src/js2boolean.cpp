/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*-
 *
 * The contents of this file are subject to the Netscape Public
 * License Version 1.1 (the "License"); you may not use this file
 * except in compliance with the License. You may obtain a copy of
 * the License at http://www.mozilla.org/NPL/
 *
 * Software distributed under the License is distributed on an "AS
 * IS" basis, WITHOUT WARRANTY OF ANY KIND, either express or
 * implied. See the License for the specific language governing
 * rights and limitations under the License.
 *
 * The Original Code is the JavaScript 2 Prototype.
 *
 * The Initial Developer of the Original Code is Netscape
 * Communications Corporation.  Portions created by Netscape are
 * Copyright (C) 1998 Netscape Communications Corporation. All
 * Rights Reserved.
 *
 * Contributor(s): 
 *
 * Alternatively, the contents of this file may be used under the
 * terms of the GNU Public License (the "GPL"), in which case the
 * provisions of the GPL are applicable instead of those above.
 * If you wish to allow use of your version of this file only
 * under the terms of the GPL and not to allow others to use your
 * version of this file under the NPL, indicate your decision by
 * deleting the provisions above and replace them with the notice
 * and other provisions required by the GPL.  If you do not delete
 * the provisions above, a recipient may use your version of this
 * file under either the NPL or the GPL.
 */

#ifdef _WIN32
#include "msvc_pragma.h"
#endif

#include <algorithm>
#include <list>
#include <map>
#include <stack>

#include "world.h"
#include "utilities.h"
#include "js2value.h"
#include "numerics.h"
#include "reader.h"
#include "parser.h"
#include "regexp.h"
#include "js2engine.h"
#include "bytecodecontainer.h"
#include "js2metadata.h"

namespace JavaScript {    
namespace MetaData {


    js2val Boolean_Constructor(JS2Metadata *meta, const js2val thisValue, js2val argv[], uint32 argc)
    {   
        js2val thatValue = OBJECT_TO_JS2VAL(new BooleanInstance(meta->booleanClass->prototype, meta->booleanClass));
        BooleanInstance *boolInst = checked_cast<BooleanInstance *>(JS2VAL_TO_OBJECT(thatValue));
        JS2Object::RootIterator ri = JS2Object::addRoot(&boolInst);

        if (argc > 0)
            boolInst->mValue = meta->toBoolean(argv[0]);
        else
            boolInst->mValue = false;
        JS2Object::removeRoot(ri);
        return thatValue;
    }
    
    static js2val Boolean_Call(JS2Metadata *meta, const js2val thisValue, js2val argv[], uint32 argc)
    {   
        if (argc > 0)
            return BOOLEAN_TO_JS2VAL(meta->toBoolean(argv[0]));
        else
            return JS2VAL_FALSE;
    }
    
    static js2val Boolean_toString(JS2Metadata *meta, const js2val thisValue, js2val * /*argv*/, uint32 /*argc*/)
    {
        if (!JS2VAL_IS_OBJECT(thisValue) 
                || (JS2VAL_TO_OBJECT(thisValue)->kind != PrototypeInstanceKind)
                || ((checked_cast<PrototypeInstance *>(JS2VAL_TO_OBJECT(thisValue)))->type != meta->booleanClass))
            meta->reportError(Exception::typeError, "Boolean.toString called on something other than a boolean thing", meta->engine->errorPos());
        BooleanInstance *boolInst = checked_cast<BooleanInstance *>(JS2VAL_TO_OBJECT(thisValue));
        return (boolInst->mValue) ? meta->engine->allocString(meta->engine->true_StringAtom) : meta->engine->allocString(meta->engine->false_StringAtom);
    }

    static js2val Boolean_valueOf(JS2Metadata *meta, const js2val thisValue, js2val * /*argv*/, uint32 /*argc*/)
    {
        if (!JS2VAL_IS_OBJECT(thisValue) 
                || (JS2VAL_TO_OBJECT(thisValue)->kind != PrototypeInstanceKind)
                || ((checked_cast<PrototypeInstance *>(JS2VAL_TO_OBJECT(thisValue)))->type != meta->booleanClass))
            meta->reportError(Exception::typeError, "Boolean.valueOf called on something other than a boolean thing", meta->engine->errorPos());
        BooleanInstance *boolInst = checked_cast<BooleanInstance *>(JS2VAL_TO_OBJECT(thisValue));
        return (boolInst->mValue) ? JS2VAL_TRUE : JS2VAL_FALSE;
    }

    void initBooleanObject(JS2Metadata *meta)
    {
        meta->booleanClass->construct = Boolean_Constructor;
        meta->booleanClass->call = Boolean_Call;

        typedef struct {
            char *name;
            uint16 length;
            NativeCode *code;
        } PrototypeFunction;

        PrototypeFunction prototypeFunctions[] =
        {
            { "toString",            0, Boolean_toString },
            { "valueOf",             0, Boolean_valueOf  },
            { NULL }
        };


        NamespaceList publicNamespaceList;
        publicNamespaceList.push_back(meta->publicNamespace);

        meta->booleanClass->prototype = new BooleanInstance(meta->objectClass->prototype, meta->booleanClass);
        

        // Adding "prototype" & "length" as static members of the class - not dynamic properties; XXX
        meta->env->addFrame(meta->booleanClass);
            Variable *v = new Variable(meta->booleanClass, OBJECT_TO_JS2VAL(meta->booleanClass->prototype), true);
            meta->defineLocalMember(meta->env, meta->engine->prototype_StringAtom, &publicNamespaceList, Attribute::NoOverride, false, ReadWriteAccess, v, 0);
            v = new Variable(meta->numberClass, INT_TO_JS2VAL(1), true);
            meta->defineLocalMember(meta->env, meta->engine->length_StringAtom, &publicNamespaceList, Attribute::NoOverride, false, ReadWriteAccess, v, 0);
        meta->env->removeTopFrame();

        // Add "constructor" as a dynamic property of the prototype
        FunctionInstance *fInst = new FunctionInstance(meta->functionClass->prototype, meta->functionClass);
        meta->writeDynamicProperty(fInst, new Multiname(meta->engine->length_StringAtom, meta->publicNamespace), true, INT_TO_JS2VAL(1), RunPhase);
        fInst->fWrap = new FunctionWrapper(true, new ParameterFrame(JS2VAL_INACCESSIBLE, true), Boolean_Constructor);
        meta->writeDynamicProperty(meta->booleanClass->prototype, new Multiname(&meta->world.identifiers["constructor"], meta->publicNamespace), true, OBJECT_TO_JS2VAL(fInst), RunPhase);


        PrototypeFunction *pf = &prototypeFunctions[0];
        while (pf->name) {
            SimpleInstance *callInst = new SimpleInstance(meta->functionClass);
            callInst->fWrap = new FunctionWrapper(true, new ParameterFrame(JS2VAL_INACCESSIBLE, true), pf->code);
    /*
    XXX not static members, since those can't be accessed from the instance
              Variable *v = new Variable(meta->functionClass, OBJECT_TO_JS2VAL(callInst), true);
              meta->defineLocalMember(&meta->env, &meta->world.identifiers[pf->name], &publicNamespaceList, Attribute::NoOverride, false, ReadWriteAccess, v, 0);
    */
            InstanceMember *m = new InstanceMethod(callInst);
            meta->defineInstanceMember(meta->booleanClass, &meta->cxt, &meta->world.identifiers[pf->name], &publicNamespaceList, Attribute::NoOverride, false, ReadWriteAccess, m, 0);

    /*
        Dynamic property of the prototype:
    */
            FunctionInstance *fInst = new FunctionInstance(meta->functionClass->prototype, meta->functionClass);
            fInst->fWrap = callInst->fWrap;
            meta->writeDynamicProperty(meta->booleanClass->prototype, new Multiname(&meta->world.identifiers[pf->name], meta->publicNamespace), true, OBJECT_TO_JS2VAL(fInst), RunPhase);
            meta->writeDynamicProperty(fInst, new Multiname(meta->engine->length_StringAtom, meta->publicNamespace), true, INT_TO_JS2VAL(pf->length), RunPhase);
            
            pf++;
        }

    }

}
}


