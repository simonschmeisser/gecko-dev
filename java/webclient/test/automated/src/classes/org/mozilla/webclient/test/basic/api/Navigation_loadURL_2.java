/* -*- Mode: C++; tab-width: 4; indent-tabs-mode: nil; c-basic-offset: 4 -*-
 * The contents of this file are subject to the Mozilla Public
 * License Version 1.1 (the "License"); you may not use this file
 * except in compliance with the License. You may obtain a copy of
 * the License at http://www.mozilla.org/MPL/
 *
 * Software distributed under the License is distributed on an "AS
 * IS" basis, WITHOUT WARRANTY OF ANY KIND, either express or
 * implied. See the License for the specific language governing
 * rights and limitations under the License.
 *
 * The Original Code is mozilla.org code.
 *
 * The Initial Developer of the Original Code is Sun Microsystems,
 * Inc. Portions created by Sun are
 * Copyright (C) 1999 Sun Microsystems, Inc. All
 * Rights Reserved.
 *
 * Contributor(s):
 * Client QA Team, St. Petersburg, Russia
 */

 
package org.mozilla.webclient.test.basic.api;

/*
 * Navigation_loadURL_2.java
 */

import org.mozilla.webclient.test.basic.*;
import org.mozilla.webclient.*;
import java.util.StringTokenizer;
import java.net.URL;
import java.net.MalformedURLException;


public class Navigation_loadURL_2 implements Test
{
    private TestContext context = null;
    private BrowserControl browserControl = null;
    private History history = null;
    private Navigation navigation = null;
    private String currentArg = null;
    private String[] pages = null;
    private boolean loading = false;

    public boolean initialize(TestContext context) {
        this.context = context;
        this.browserControl = context.getBrowserControl();
        this.pages = context.getPagesToLoad();
//        this.currentArg = (new Integer(context.getCurrentTestArguments())).intValue();
        this.currentArg = context.getCurrentTestArguments();
        try {
            navigation = (Navigation)
                browserControl.queryInterface(BrowserControl.NAVIGATION_NAME);

            history = (History)
                browserControl.queryInterface(BrowserControl.HISTORY_NAME);

        }catch (Exception e) {
            TestContext.registerFAILED("Exception: " + e.toString());
            return false;
		}
        return true;        
    }


 public void execute() {      

 if (null != navigation) {

StringTokenizer st=new StringTokenizer(currentArg,",");
int num=java.lang.Integer.parseInt(st.nextToken().trim());
String host1=st.nextToken().trim();
String host2=st.nextToken().trim();

if (!host2.equals("null")) {

  if (!loading) {
    navigation.loadURL(host2);
    System.out.println("LOAD");
    loading=true;
   }

 if (num!=2) if (context.getExecutionCount()!=2) return;
}
  
    switch (num) {
	case 0: 
            try {
      		context.setDefaultResult(TestContext.FAILED);
		context.setDefaultComment("We are trying to load URL: "+host1);

		navigation.loadURL(host1);
		
            } catch(Exception e) {
               if(e instanceof UnimplementedException)
                  TestContext.registerUNIMPLEMENTED("This method doesn't implemented");
               else
                 TestContext.registerFAILED("Exception during execution: " + e.toString());
                 return;
             }; 
	break;

	case 1: 
	case 2:
            try {
      		context.setDefaultResult(TestContext.FAILED);
		context.setDefaultComment("We are trying to load URL when another URL is loading or loaded");

  		navigation.loadURL(host1);

            }catch(Exception e) {
               if(e instanceof UnimplementedException)
                  TestContext.registerUNIMPLEMENTED("This method doesn't implemented");
               else
                 TestContext.registerFAILED("Exception during execution: " + e.toString());
                 return;
            };
	break;

      };

  TestContext.registerPASSED("Browser doesn't crashed");

     return;
  
    }	}   
}
