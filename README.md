# Zana
Zana is a wrapper for AWS SDK written in Rust.

It simplifies programmatic interation with AWS within your local environment.

Think of it as a `kubectl`-like CLI for daily operational use cases with your AWS environment.

## AWS Glue
TBC

## AWS S3
TBC

## AWS Redshift
TBC

## AWS Cost-Explorer 
The following description explains cost types that we can use as input for the `cost-explorer` API:  

1. UnblendedCost  
This is the cost dataset presented to you on the Bills page. It's the default option for analyzing cost in AWS Cost Explorer. Unblended costs represent our usage costs on the day they are charged to you. In finance terms, they represent your costs on a cash basis of accounting.  
  
2. NetUnblendedCost
This cost metric reflects the UnblendedCosts after discounts.

3. AmortizedCost  
It is used to view costs on an accrual basis rather than a cash basis. It is useful if you have purchased AWS Reservations e.g. EC2 Reserved Instances.  

4. NetAmortizedCost
This cost metric amortizes the upfront and monthly reservation fees while including discounts such as Reserved Instances volume discounts.

5. BlendedCost  
It is calculated by multiplying each account's service usage agains something called a blended rate. A blended rate is the average rate of on-demand usage, as well as Savings Plans and reservation-related usage, that is consumed by member accounts in an organization for a particular service.

6. UsageQuantity
<TBC>

# Author(s)
* Ngakan Gandhi