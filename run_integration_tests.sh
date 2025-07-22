#!/bin/bash

echo "🧪 Airtel Money API Integration Test Runner"
echo "==========================================="
echo ""

# Check if .env file exists
if [ ! -f ".env" ]; then
    echo "❌ Error: .env file not found!"
    echo "Please create a .env file with your Airtel Money credentials:"
    echo ""
    echo "AIRTEL_CLIENT_ID=your_client_id"
    echo "AIRTEL_CLIENT_SECRET=your_client_secret"
    echo "AIRTEL_PIN=your_pin"
    echo ""
    exit 1
fi

echo "✅ Found .env file with credentials"
echo ""

# Check if credentials are set
source .env
if [ -z "$AIRTEL_CLIENT_ID" ] || [ -z "$AIRTEL_CLIENT_SECRET" ] || [ -z "$AIRTEL_PIN" ]; then
    echo "❌ Error: Missing required environment variables in .env file"
    echo "Required variables: AIRTEL_CLIENT_ID, AIRTEL_CLIENT_SECRET, AIRTEL_PIN"
    exit 1
fi

echo "🔧 Environment Setup:"
echo "Client ID: ${AIRTEL_CLIENT_ID:0:8}..."
echo "Client Secret: ${AIRTEL_CLIENT_SECRET:0:8}..."
echo "PIN: ****"
echo ""

echo "📋 Available Test Suites:"
echo "1. Individual Tests (run specific test functions)"
echo "2. Full Integration Suite (run all tests sequentially)"
echo "3. Concurrent Tests (test API under load)"
echo ""

read -p "Choose test suite (1-3) or press Enter for all individual tests: " choice

case $choice in
    1)
        echo "🚀 Running individual integration tests..."
        cargo test --test live_api_tests -- --nocapture
        ;;
    2)
        echo "🚀 Running specific test..."
        echo "Available tests:"
        echo "- test_token_generation_and_refresh"
        echo "- test_account_balance"
        echo "- test_collection_ussd_push"
        echo "- test_disbursement"
        echo "- test_cash_in"
        echo "- test_cash_out"
        echo "- test_remittance_eligibility"
        echo "- test_remittance_transfer_credit"
        echo "- test_collection_refund"
        echo "- test_multiple_countries"
        echo "- test_error_handling"
        echo "- test_concurrent_requests"
        read -p "Enter test name: " test_name
        cargo test --test live_api_tests $test_name -- --nocapture
        ;;
    3)
        echo "🚀 Running concurrent tests..."
        cargo test --test live_api_tests test_concurrent_requests -- --nocapture
        ;;
    *)
        echo "🚀 Running all individual integration tests..."
        cargo test --test live_api_tests -- --nocapture
        ;;
esac

echo ""
echo "✅ Integration tests completed!"
echo ""
echo "📝 Notes:"
echo "- Tests use the Sandbox environment for safety"
echo "- Some tests may fail if the sandbox has restrictions"
echo "- Check the output above for detailed results"
echo "- For production testing, modify the Environment in the test file"