package com.solanasos.emergency

import android.app.Dialog
import android.content.Context
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.ImageView
import android.widget.TextView
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.google.android.material.bottomsheet.BottomSheetDialogFragment

/**
 * Wallet Selection Dialog for Solana Mobile Wallet Adapter
 * Shows available wallets and handles selection
 */
class WalletSelectionDialog : BottomSheetDialogFragment() {

    interface WalletSelectionListener {
        fun onWalletSelected(walletType: String)
        fun onWalletSelectionCancelled()
    }

    private var listener: WalletSelectionListener? = null

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        return inflater.inflate(R.layout.dialog_wallet_selection, container, false)
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        val recyclerView = view.findViewById<RecyclerView>(R.id.rvWallets)
        recyclerView.layoutManager = LinearLayoutManager(context)
        
        val wallets = getAvailableWallets()
        val adapter = WalletAdapter(wallets) { wallet ->
            listener?.onWalletSelected(wallet.type)
            dismiss()
        }
        recyclerView.adapter = adapter

        // Cancel button
        view.findViewById<Button>(R.id.btnCancel).setOnClickListener {
            listener?.onWalletSelectionCancelled()
            dismiss()
        }
    }

    private fun getAvailableWallets(): List<WalletInfo> {
        return listOf(
            WalletInfo("phantom", "Phantom", "Most popular Solana wallet", R.drawable.ic_wallet_phantom),
            WalletInfo("solflare", "Solflare", "Fast and secure Solana wallet", R.drawable.ic_wallet_solflare),
            WalletInfo("backpack", "Backpack", "Professional Solana wallet", R.drawable.ic_wallet_backpack),
            WalletInfo("exodus", "Exodus", "Multi-chain wallet with Solana support", R.drawable.ic_wallet_exodus),
            WalletInfo("trust", "Trust Wallet", "Binance's secure wallet", R.drawable.ic_wallet_trust)
        )
    }

    fun setListener(listener: WalletSelectionListener) {
        this.listener = listener
    }

    data class WalletInfo(
        val type: String,
        val name: String,
        val description: String,
        val iconResId: Int
    )

    private inner class WalletAdapter(
        private val wallets: List<WalletInfo>,
        private val onWalletSelected: (WalletInfo) -> Unit
    ) : RecyclerView.Adapter<WalletAdapter.WalletViewHolder>() {

        inner class WalletViewHolder(itemView: View) : RecyclerView.ViewHolder(itemView) {
            private val ivIcon: ImageView = itemView.findViewById(R.id.ivWalletIcon)
            private val tvName: TextView = itemView.findViewById(R.id.tvWalletName)
            private val tvDescription: TextView = itemView.findViewById(R.id.tvWalletDescription)

            fun bind(wallet: WalletInfo) {
                ivIcon.setImageResource(wallet.iconResId)
                tvName.text = wallet.name
                tvDescription.text = wallet.description

                itemView.setOnClickListener {
                    onWalletSelected(wallet)
                }
            }
        }

        override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): WalletViewHolder {
            val view = LayoutInflater.from(parent.context)
                .inflate(R.layout.item_wallet, parent, false)
            return WalletViewHolder(view)
        }

        override fun onBindViewHolder(holder: WalletViewHolder, position: Int) {
            holder.bind(wallets[position])
        }

        override fun getItemCount(): Int = wallets.size
    }

    companion object {
        fun newInstance(): WalletSelectionDialog {
            return WalletSelectionDialog()
        }
    }
} 