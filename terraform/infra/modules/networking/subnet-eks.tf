# only ipv4
# for eks subnet, mimicking private subnet from
# https://github.com/terraform-aws-modules/terraform-aws-vpc/blob/cf18c37591f860908e2223b4f488787e8a5f74f3/main.tf

resource "aws_subnet" "eks_system" {
  count = local.eks_subnet_create ? local.eks_subnet_len : 0

  assign_ipv6_address_on_creation = false
  availability_zone               = length(regexall("^[a-z]{2}-", element(local.azs, count.index))) > 0 ? element(local.azs, count.index) : null
  availability_zone_id            = length(regexall("^[a-z]{2}-", element(local.azs, count.index))) == 0 ? element(local.azs, count.index) : null
  cidr_block                      = local.subnet_cidrs[element(local.eks_cidr_index, count.index)]
  vpc_id                          = module.vpc.vpc_id

  tags = merge(
    {
      Name = try(
        format("${local.name}-${local.eks_subnet_suffix}-%s", element(local.azs, count.index))
      )
    },
    {
      "kubernetes.io/cluster/my-eks-cluster" = "shared",
      "kubernetes.io/role/internal-elb"      = 1
    },
    var.tags
  )
}

resource "aws_route_table" "eks_system" {
  count = length(module.vpc.nat_ids)

  vpc_id = module.vpc.vpc_id

  tags = merge(
    {
      "Name" = local.single_nat_gateway ? "${local.name}-${local.eks_subnet_suffix}" : format(
        "${local.name}-${local.eks_subnet_suffix}-%s",
        element(local.azs, count.index),
      )
    },
    var.tags
  )
}

resource "aws_route_table_association" "eks_system" {
  count = local.eks_subnet_create ? local.eks_subnet_len : 0

  subnet_id = element(aws_subnet.eks_system[*].id, count.index)
  route_table_id = element(
    aws_route_table.eks_system[*].id,
    local.single_nat_gateway ? 0 : count.index,
  )
}

resource "aws_route" "eks_nat_gateway" {
  count = local.nat_gateway_count

  route_table_id         = element(aws_route_table.eks_system[*].id, count.index)
  destination_cidr_block = "0.0.0.0/0"
  nat_gateway_id         = module.vpc.natgw_ids[count.index]

  timeouts {
    create = "5m"
  }
}
