---
name: API Integration Issue
about: Report issues with Airtel Money API integration
title: '[API] '
labels: api-integration
assignees: ''
---

# API Integration Issue

## API Endpoint
<!-- Which Airtel Money API endpoint is affected? -->
- [ ] Authentication (`/auth/oauth2/token`)
- [ ] Collections - USSD Push (`/merchant/v1/payments/`)
- [ ] Collections - Refund (`/standard/v1/payments/refund`)
- [ ] Collections - Status (`/standard/v1/payments/status/{id}`)
- [ ] Disbursements (`/standard/v1/disbursements/`)
- [ ] Remittances - Eligibility (`/standard/v1/remittances/`)
- [ ] Remittances - Transfer (`/standard/v1/remittances/`)
- [ ] Cash In (`/standard/v1/cashin/`)
- [ ] Cash Out (`/standard/v1/cashout/`)
- [ ] Account Balance (`/standard/v1/users/balance`)
- [ ] Account KYC (`/standard/v1/users/`)
- [ ] Other: ___________

## Environment
<!-- Which environment are you using? -->
- [ ] Sandbox (`https://openapiuat.airtel.africa`)
- [ ] Production (`https://openapi.airtel.africa`)

## Country/Currency
<!-- Which country and currency are you testing with? -->
- **Country**: <!-- e.g., Kenya (KE) -->
- **Currency**: <!-- e.g., KES -->

## Issue Type
<!-- What type of issue are you experiencing? -->
- [ ] Authentication failure
- [ ] Request format error
- [ ] Response parsing error
- [ ] Unexpected API behavior
- [ ] Timeout issues
- [ ] Rate limiting
- [ ] HTTP status code issues
- [ ] Webhook/callback issues

## Request Details
<!-- Provide details about your request (remove sensitive data) -->
```json
{
  "method": "POST",
  "url": "https://openapiuat.airtel.africa/...",
  "headers": {
    "Authorization": "Bearer [REDACTED]",
    "Content-Type": "application/json",
    "X-Country": "KE",
    "X-Currency": "KES"
  },
  "body": {
    // Your request body (remove sensitive data)
  }
}
```

## Response Details
<!-- What response did you receive? -->
```json
{
  "status": 400,
  "headers": {
    // Response headers
  },
  "body": {
    // Response body
  }
}
```

## Expected Behavior
<!-- What did you expect to happen? -->

## Code Sample
<!-- Provide a minimal code sample that reproduces the issue -->
```rust
use airtel_rs::{AirtelMoney, Environment, Country};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let airtel = AirtelMoney::new(Environment::Sandbox, Country::Kenya);
    // Your code here
    Ok(())
}
```

## Error Messages
<!-- Include any error messages -->
```
Paste error messages here
```

## Network/Connectivity
<!-- Any network-related details -->
- [ ] Using proxy
- [ ] Using VPN
- [ ] Corporate firewall
- [ ] Rate limiting encountered
- [ ] Timeout issues

## Credentials Status
<!-- Have you verified your credentials? -->
- [ ] Credentials work in Postman/curl
- [ ] Credentials are for the correct environment
- [ ] Client ID and secret are valid
- [ ] Permissions are correctly configured

## Additional Context
<!-- Any other relevant information -->

## Frequency
<!-- How often does this issue occur? -->
- [ ] Always (100%)
- [ ] Frequently (>75%)
- [ ] Sometimes (25-75%)
- [ ] Rarely (<25%)
- [ ] First time encountering

## Impact
<!-- How is this affecting your integration? -->
- [ ] Blocking development completely
- [ ] Blocking specific functionality
- [ ] Causing intermittent failures
- [ ] Minor inconvenience

## Temporary Workarounds
<!-- Have you found any workarounds? -->

## Related Issues
<!-- Link to any related issues or discussions -->