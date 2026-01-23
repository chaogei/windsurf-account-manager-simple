#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Configuration
const SRC_DIR = path.join(__dirname, '../src');
const LOCALES_DIR = path.join(__dirname, '../src/i18n/locales');
const LANGUAGES = ['en', 'fr', 'es', 'zh'];

// Load translation files
function loadTranslations() {
  const translations = {};
  
  LANGUAGES.forEach(lang => {
    const filePath = path.join(LOCALES_DIR, `${lang}.json`);
    try {
      const content = fs.readFileSync(filePath, 'utf8');
      translations[lang] = JSON.parse(content);
    } catch (error) {
      console.error(`Error loading ${lang}.json:`, error.message);
      translations[lang] = {};
    }
  });
  
  return translations;
}

// Extract all translation keys from an object
function extractKeys(obj, prefix = '') {
  const keys = new Set();
  
  function traverse(current, currentPrefix) {
    if (typeof current === 'object' && current !== null) {
      Object.keys(current).forEach(key => {
        const newPrefix = currentPrefix ? `${currentPrefix}.${key}` : key;
        traverse(current[key], newPrefix);
      });
    } else {
      keys.add(currentPrefix);
    }
  }
  
  traverse(obj, prefix);
  return keys;
}

// Find all $t() calls in Vue/TS/JS files
function findTranslationCalls(dir) {
  const calls = new Set();
  
  function processFile(filePath) {
    try {
      const content = fs.readFileSync(filePath, 'utf8');
      
      // Find $t('key') and $t("key") patterns
      const tMatches = content.match(/\$t\s*\(\s*['"`]([^'"`]+)['"`]\s*\)/g);
      if (tMatches) {
        tMatches.forEach(match => {
          const keyMatch = match.match(/\$t\s*\(\s*['"`]([^'"`]+)['"`]\s*\)/);
          if (keyMatch) {
            calls.add(keyMatch[1]);
          }
        });
      }
      
      // Find t('key') and t("key") patterns
      const directMatches = content.match(/(?<!\$)t\s*\(\s*['"`]([^'"`]+)['"`]\s*\)/g);
      if (directMatches) {
        directMatches.forEach(match => {
          const keyMatch = match.match(/t\s*\(\s*['"`]([^'"`]+)['"`]\s*\)/);
          if (keyMatch) {
            calls.add(keyMatch[1]);
          }
        });
      }
    } catch (error) {
      console.error(`Error processing ${filePath}:`, error.message);
    }
  }
  
  function processDirectory(dirPath) {
    try {
      const items = fs.readdirSync(dirPath);
      
      for (const item of items) {
        const itemPath = path.join(dirPath, item);
        const stat = fs.statSync(itemPath);
        
        if (stat.isDirectory() && !item.startsWith('.') && item !== 'node_modules') {
          processDirectory(itemPath);
        } else if (stat.isFile() && /\.(vue|ts|js)$/.test(item)) {
          processFile(itemPath);
        }
      }
    } catch (error) {
      console.error(`Error processing directory ${dirPath}:`, error.message);
    }
  }
  
  processDirectory(dir);
  return calls;
}

// Find hardcoded strings (Chinese characters)
function findHardcodedStrings(dir) {
  const hardcodedStrings = [];
  
  function processFile(filePath) {
    try {
      const content = fs.readFileSync(filePath, 'utf8');
      
      // Find Chinese characters that are not in comments
      const lines = content.split('\n');
      lines.forEach((line, index) => {
        // Skip comments
        const withoutComments = line.split('//')[0].split('/*')[0];
        
        // Find Chinese characters
        const chineseMatches = withoutComments.match(/[\u4e00-\u9fff]+/g);
        if (chineseMatches) {
          chineseMatches.forEach(chineseText => {
            hardcodedStrings.push({
              file: filePath,
              line: index + 1,
              text: chineseText,
              context: line.trim()
            });
          });
        }
      });
    } catch (error) {
      console.error(`Error processing ${filePath}:`, error.message);
    }
  }
  
  function processDirectory(dirPath) {
    try {
      const items = fs.readdirSync(dirPath);
      
      for (const item of items) {
        const itemPath = path.join(dirPath, item);
        const stat = fs.statSync(itemPath);
        
        if (stat.isDirectory() && !item.startsWith('.') && item !== 'node_modules') {
          processDirectory(itemPath);
        } else if (stat.isFile() && /\.(vue|ts|js)$/.test(item)) {
          processFile(itemPath);
        }
      }
    } catch (error) {
      console.error(`Error processing directory ${dirPath}:`, error.message);
    }
  }
  
  processDirectory(dir);
  return hardcodedStrings;
}

// Main analysis
function analyzeTranslations() {
  console.log('🔍 Analyzing translations...\n');
  
  // Load translations
  const translations = loadTranslations();
  
  // Get all keys from each language
  const languageKeys = {};
  LANGUAGES.forEach(lang => {
    languageKeys[lang] = extractKeys(translations[lang]);
  });
  
  // Find all translation calls in code
  const usedKeys = findTranslationCalls(SRC_DIR);
  
  // Find hardcoded strings
  const hardcodedStrings = findHardcodedStrings(SRC_DIR);
  
  // Analyze missing translations
  console.log('📊 Translation Analysis Results:\n');
  
  LANGUAGES.forEach(lang => {
    const missing = [...usedKeys].filter(key => !languageKeys[lang].has(key));
    if (missing.length > 0) {
      console.log(`❌ ${lang.toUpperCase()}: ${missing.length} missing translations`);
      missing.forEach(key => {
        console.log(`   - ${key}`);
      });
      console.log('');
    } else {
      console.log(`✅ ${lang.toUpperCase()}: All translations found`);
    }
  });
  
  // Show unused translations
  console.log('\n📋 Unused Translations:\n');
  LANGUAGES.forEach(lang => {
    const unused = [...languageKeys[lang]].filter(key => !usedKeys.has(key));
    if (unused.length > 0) {
      console.log(`${lang.toUpperCase()}: ${unused.length} unused keys`);
      unused.slice(0, 10).forEach(key => {
        console.log(`   - ${key}`);
      });
      if (unused.length > 10) {
        console.log(`   ... and ${unused.length - 10} more`);
      }
      console.log('');
    }
  });
  
  // Show hardcoded strings
  if (hardcodedStrings.length > 0) {
    console.log('🚨 Hardcoded Strings Found:\n');
    hardcodedStrings.slice(0, 20).forEach(item => {
      console.log(`${item.file}:${item.line}`);
      console.log(`  Text: "${item.text}"`);
      console.log(`  Context: ${item.context}`);
      console.log('');
    });
    if (hardcodedStrings.length > 20) {
      console.log(`... and ${hardcodedStrings.length - 20} more`);
    }
  } else {
    console.log('✅ No hardcoded strings found');
  }
  
  // Summary
  console.log('\n📈 Summary:');
  console.log(`- Total translation keys used: ${usedKeys.size}`);
  console.log(`- Hardcoded strings found: ${hardcodedStrings.length}`);
  console.log(`- Languages analyzed: ${LANGUAGES.join(', ')}`);
  
  // Save detailed report
  const report = {
    timestamp: new Date().toISOString(),
    usedKeys: Array.from(usedKeys),
    missingTranslations: {},
    unusedTranslations: {},
    hardcodedStrings: hardcodedStrings
  };
  
  LANGUAGES.forEach(lang => {
    report.missingTranslations[lang] = [...usedKeys].filter(key => !languageKeys[lang].has(key));
    report.unusedTranslations[lang] = [...languageKeys[lang]].filter(key => !usedKeys.has(key));
  });
  
  const reportPath = path.join(__dirname, '../translation-report.json');
  fs.writeFileSync(reportPath, JSON.stringify(report, null, 2));
  console.log(`\n📄 Detailed report saved to: ${reportPath}`);
}

// Run analysis
if (require.main === module) {
  analyzeTranslations();
}

module.exports = { analyzeTranslations };
