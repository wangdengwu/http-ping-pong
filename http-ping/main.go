package main

import (
	"context"
	"errors"
	"fmt"
	"net/http"
	"net/http/httputil"
	"net/url"
	"os"
	"os/signal"
	"time"
)

func handler(w http.ResponseWriter, r *http.Request) {
	xHost := r.Header.Get("x-host")
	if xHost == "" {
		http.Error(w, "x-host header is missing", http.StatusBadRequest)
		return
	}
	targetURL, err := url.Parse(xHost)
	if err != nil {
		http.Error(w, "Invalid x-host header", http.StatusBadRequest)
		return
	}

	proxy := httputil.NewSingleHostReverseProxy(targetURL)
	proxy.ServeHTTP(w, r)
}

func main() {
	srv := &http.Server{
		Addr:    ":9999",
		Handler: http.HandlerFunc(handler),
	}

	go func() {
		fmt.Println("Server is running on port 8080")
		if err := srv.ListenAndServe(); err != nil && !errors.Is(err, http.ErrServerClosed) {
			fmt.Printf("Error: %v\n", err)
		}
	}()

	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	<-c

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	if err := srv.Shutdown(ctx); err != nil {
		fmt.Printf("Error shutting down server: %v\n", err)
	}

	fmt.Println("Server gracefully stopped")
}
