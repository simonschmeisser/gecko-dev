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
 * BookmarkEntry_getProperties.java
 */

import org.mozilla.webclient.test.basic.*;
import org.mozilla.webclient.*;
import java.util.StringTokenizer;
import java.util.*;
import java.net.URL;
import java.net.MalformedURLException;
import javax.swing.tree.*;

public class BookmarkEntry_getProperties implements Test
{
    
    private TestContext context = null;
    private BrowserControl browserControl = null;
    private Bookmarks bookmarks = null;
    private int currentArg;
    private CurrentPage curPage = null;
    private String[] pages = null;
    private BookmarkEntry bookmarkEntry, be = null;
    private TreeModel bookmarksTree = null;
    private TreeNode node = null;

    public boolean initialize(TestContext context) {
        this.context = context;
        this.browserControl = context.getBrowserControl();
        this.pages = context.getPagesToLoad();
        this.currentArg = (new Integer(context.getCurrentTestArguments())).intValue();
        try {
            bookmarks = (Bookmarks)
                browserControl.queryInterface(BrowserControl.BOOKMARKS_NAME);
        }catch (Exception e) {
            TestContext.registerFAILED("Exception: " + e.toString());
            return false;
        }
        return true;        
    }
public void execute() {      

  if (bookmarks==null) return;


	context.setDefaultResult(TestContext.FAILED);
	context.setDefaultComment("We are trying to get properties in test case N"+currentArg);
	
  switch (currentArg) {
   case 0:
       try {
        bookmarksTree = bookmarks.getBookmarks();
	node=(TreeNode)bookmarksTree.getRoot();

	bookmarkEntry=(BookmarkEntry)node.getChildAt(0).getChildAt(1);

       } catch(Exception e) {
       if (e instanceof UnimplementedException) TestContext.registerUNIMPLEMENTED("This method doesn't implemented");
         else TestContext.registerFAILED("Exception during execution: " + e.toString());
       }
	String real="{URL="+node.getChildAt(0).getChildAt(1)+"}";
       if (bookmarkEntry.getProperties().equals(real)) TestContext.registerPASSED("getProperties return correct Properties="+bookmarkEntry.getProperties());
         else TestContext.registerFAILED("getProperties return empty or bad Properties="+bookmarkEntry.getProperties());
       break;

   case 1:
       try {

        bookmarkEntry = bookmarks.newBookmarkEntry("http://java.sun.com");

       } catch(Exception e) {
       if (e instanceof UnimplementedException) TestContext.registerUNIMPLEMENTED("This method doesn't implemented");
         else TestContext.registerFAILED("Exception during execution: " + e.toString());
       }
       if (bookmarkEntry.getProperties().toString().equals("{URL=http://java.sun.com}")) TestContext.registerPASSED("getProperties return correct Properties="+bookmarkEntry.getProperties());
        else TestContext.registerFAILED("getProperties return empty or bad Properties="+bookmarkEntry.getProperties());
       break;

   case 2:
       try {

        bookmarkEntry = bookmarks.newBookmarkFolder("My bookmarks");

       } catch(Exception e) {
       if (e instanceof UnimplementedException) TestContext.registerUNIMPLEMENTED("This method doesn't implemented");
         else TestContext.registerFAILED("Exception during execution: " + e.toString());
       }
       if (bookmarkEntry.getProperties().toString().equals("{NAME=My bookmarks}")) TestContext.registerPASSED("getProperties return correct Properties="+bookmarkEntry.getProperties());
        else TestContext.registerFAILED("getProperties return empty or bad Properties"+bookmarkEntry.getProperties());
       break;
}  } 
}




