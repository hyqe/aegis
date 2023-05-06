package main

import (
	"fmt"
	"net/http"
	"os"
)

func main() {
	// get port more change, EVEN MORE changes
	addr := fmt.Sprintf(":%v", os.Getenv("PORT"))

	fmt.Println("Running on ", addr)
	http.ListenAndServe(addr, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "Your Data is definitely safe & secure with the Hyqe Organization...!")
	}))

}
