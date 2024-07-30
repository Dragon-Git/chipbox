/*
Copyright © 2024 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"
	"log"
	"os/exec"

	"github.com/spf13/cobra"
)

// waveCmd represents the wave command
var waveCmd = &cobra.Command{
	Use:   "wave",
	Short: "open fsdb file use verdi",
	Long:  ``,
	Run: func(cmd *cobra.Command, args []string) {
		c := exec.Command("ls")
		err := c.Run()
		if err != nil {
			log.Fatal(err)
		}
		message, _ := cmd.Flags().GetString("filelist")
		fmt.Println(message)
		fmt.Println("wave called")
	},
}

func init() {
	rootCmd.AddCommand(waveCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// waveCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	waveCmd.Flags().StringP("filelist", "f", "", "get a filelist")
}
