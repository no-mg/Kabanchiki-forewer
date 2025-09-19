package handlers

import (
	"net/http"
	"time"

	"github.com/labstack/echo/v4"
)

func HelloStatic(c echo.Context) error {
	return c.Render(http.StatusOK, "index.html", nil)
}

func Hello(c echo.Context) error {
	return c.JSON(http.StatusOK, map[string]interface{}{
		"message": "HEllo from backend!",
		"status":  "success",
		"data": map[string]interface{}{
			"timestamp": time.Now().Unix(),
			"framework": "Echo",
		},
	})
}
