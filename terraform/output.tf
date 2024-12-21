output "api_gateway_url" {
  description = "URL for invoking API Gateway."
  value       = aws_apigatewayv2_stage.websocket.invoke_url
}

