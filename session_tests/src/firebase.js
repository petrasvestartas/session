// Firebase configuration for Session Test Viewer
// This file initializes Firebase and provides a function to fetch the Claude API key

import { initializeApp } from 'firebase/app'
import { getFirestore, doc, getDoc } from 'firebase/firestore'

// Firebase configuration
const firebaseConfig = {
  apiKey: "AIzaSyBJqPKN_xQbqd0WRbp0ZG_X5xZOEMnKL54",
  authDomain: "session-4c9e7.firebaseapp.com",
  projectId: "session-4c9e7",
  storageBucket: "session-4c9e7.firebasestorage.app",
  messagingSenderId: "1043857826184",
  appId: "1:1043857826184:web:372ae8707befb2bc8f5a3d",
  measurementId: "G-9EJTNJ5V6T"
}

// Initialize Firebase
let app = null
let db = null

export function initFirebase() {
  if (!app) {
    app = initializeApp(firebaseConfig)
    db = getFirestore(app)
  }
  return { app, db }
}

// Fetch Claude API key from Firestore
// Collection: claude, Document: S5Augcgl1s61k1ss3ajz, Field: petras
export async function getClaudeApiKey() {
  try {
    const { db } = initFirebase()

    const docRef = doc(db, 'claude', 'S5Augcgl1s61k1ss3ajz')
    const docSnap = await getDoc(docRef)

    if (docSnap.exists()) {
      const data = docSnap.data()
      const apiKey = data.petras

      if (apiKey && apiKey.startsWith('sk-ant-')) {
        return { key: apiKey, error: null }
      } else {
        return { key: null, error: 'Invalid API key format in Firestore' }
      }
    } else {
      return { key: null, error: 'API key document not found in Firestore' }
    }
  } catch (error) {
    console.error('Firebase error:', error)
    return { key: null, error: `Firebase error: ${error.message}` }
  }
}
