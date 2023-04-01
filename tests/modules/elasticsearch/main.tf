resource "aws_elasticsearch_domain" "main" {
  domain_name           = var.cluster_name
  elasticsearch_version = "7.10"

  cluster_config {
    instance_type = "t3.small.elasticsearch"
  }

  ebs_options {
    ebs_enabled = true
    volume_size = 50
  }
}