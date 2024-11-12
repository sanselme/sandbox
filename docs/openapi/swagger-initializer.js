window.onload = function() {
  //<editor-fold desc="Changeable Configuration Block">

  // the following lines will be replaced by docker/configurator, when it runs in a docker-container
  window.ui = SwaggerUIBundle({
    urls: [{"url":"v1/hello.swagger.json","name":"v1/hello.swagger.json"},{"url":"v1alpha1/cloudevent.swagger.json","name":"v1alpha1/cloudevent.swagger.json"},{"url":"v1alpha1/route_guide.swagger.json","name":"v1alpha1/route_guide.swagger.json"},{"url":"v1alpha1/health.swagger.json","name":"v1alpha1/health.swagger.json"},],
    dom_id: '#swagger-ui',
    deepLinking: true,
    presets: [
      SwaggerUIBundle.presets.apis,
      SwaggerUIStandalonePreset
    ],
    plugins: [
      SwaggerUIBundle.plugins.DownloadUrl
    ],
    layout: "StandaloneLayout"
  });

  //</editor-fold>
};
