package main

import (
	"html/template"
	"io"
	"log"
	"server/database"
	"server/handlers"
	"time"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

type TemplateRenderer struct {
	templates *template.Template
}

func (t *TemplateRenderer) Render(w io.Writer, name string, data interface{}, c echo.Context) error {
	return t.templates.ExecuteTemplate(w, name, data)
}

func main() {
	time.Sleep(10 * time.Second)
	if err := database.Init(); err != nil {
		log.Printf("Database initialization failed: %v", err)
		log.Println("Continuing without database connection...")
	}
	e := echo.New()
	templates := template.Must(template.ParseFiles("templates/hello/index.html"))

	e.Renderer = &TemplateRenderer{templates}
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	e.Use(middleware.CORS())
	
	// Serve static files
	e.Static("/static", "templates")
	
	e.GET("/api/hello", handlers.Hello)
	e.GET("/hello", handlers.HelloStatic)
	e.Logger.Fatal(e.Start("0.0.0.0:8080"))
}
