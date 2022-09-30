#[doc = "Register `SMR_SMCI` reader"]
pub struct R(crate::R<SMR_SMCI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMR_SMCI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMR_SMCI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMR_SMCI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMR_SMCI` writer"]
pub struct W(crate::W<SMR_SMCI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMR_SMCI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SMR_SMCI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMR_SMCI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKS` reader - Clock Select"]
pub type CKS_R = crate::FieldReader<u8, CKS_A>;
#[doc = "Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKS_A {
    #[doc = "0: PCLK clock (n = 0)"]
    _00 = 0,
    #[doc = "1: PCLK/4 clock (n = 1)"]
    _01 = 1,
    #[doc = "2: PCLK/16 clock (n = 2)"]
    _10 = 2,
    #[doc = "3: PCLK/64 clock (n = 3)"]
    _11 = 3,
}
impl From<CKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CKS_A) -> Self {
        variant as _
    }
}
impl CKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKS_A {
        match self.bits {
            0 => CKS_A::_00,
            1 => CKS_A::_01,
            2 => CKS_A::_10,
            3 => CKS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CKS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CKS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CKS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CKS_A::_11
    }
}
#[doc = "Field `CKS` writer - Clock Select"]
pub type CKS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SMR_SMCI_SPEC, u8, CKS_A, 2, O>;
impl<'a, const O: u8> CKS_W<'a, O> {
    #[doc = "PCLK clock (n = 0)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CKS_A::_00)
    }
    #[doc = "PCLK/4 clock (n = 1)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CKS_A::_01)
    }
    #[doc = "PCLK/16 clock (n = 2)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CKS_A::_10)
    }
    #[doc = "PCLK/64 clock (n = 3)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CKS_A::_11)
    }
}
#[doc = "Field `BCP` reader - Base Clock Pulse"]
pub type BCP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BCP` writer - Base Clock Pulse"]
pub type BCP_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SMR_SMCI_SPEC, u8, u8, 2, O>;
#[doc = "Field `PM` reader - Parity Mode"]
pub type PM_R = crate::BitReader<PM_A>;
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PM_A {
    #[doc = "0: Even parity"]
    _0 = 0,
    #[doc = "1: Odd parity"]
    _1 = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::_0,
            true => PM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PM_A::_1
    }
}
#[doc = "Field `PM` writer - Parity Mode"]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u8, SMR_SMCI_SPEC, PM_A, O>;
impl<'a, const O: u8> PM_W<'a, O> {
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PM_A::_0)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PM_A::_1)
    }
}
#[doc = "Field `PE` reader - Parity Enable"]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - Parity Enable"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SMR_SMCI_SPEC, bool, O>;
#[doc = "Field `BLK` reader - Block Transfer Mode"]
pub type BLK_R = crate::BitReader<BLK_A>;
#[doc = "Block Transfer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLK_A {
    #[doc = "0: Normal mode operation"]
    _0 = 0,
    #[doc = "1: Block transfer mode operation"]
    _1 = 1,
}
impl From<BLK_A> for bool {
    #[inline(always)]
    fn from(variant: BLK_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK_A {
        match self.bits {
            false => BLK_A::_0,
            true => BLK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLK_A::_1
    }
}
#[doc = "Field `BLK` writer - Block Transfer Mode"]
pub type BLK_W<'a, const O: u8> = crate::BitWriter<'a, u8, SMR_SMCI_SPEC, BLK_A, O>;
impl<'a, const O: u8> BLK_W<'a, O> {
    #[doc = "Normal mode operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLK_A::_0)
    }
    #[doc = "Block transfer mode operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLK_A::_1)
    }
}
#[doc = "Field `GM` reader - GSM Mode"]
pub type GM_R = crate::BitReader<GM_A>;
#[doc = "GSM Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GM_A {
    #[doc = "0: Normal mode operation"]
    _0 = 0,
    #[doc = "1: GSM mode operation"]
    _1 = 1,
}
impl From<GM_A> for bool {
    #[inline(always)]
    fn from(variant: GM_A) -> Self {
        variant as u8 != 0
    }
}
impl GM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GM_A {
        match self.bits {
            false => GM_A::_0,
            true => GM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GM_A::_1
    }
}
#[doc = "Field `GM` writer - GSM Mode"]
pub type GM_W<'a, const O: u8> = crate::BitWriter<'a, u8, SMR_SMCI_SPEC, GM_A, O>;
impl<'a, const O: u8> GM_W<'a, O> {
    #[doc = "Normal mode operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GM_A::_0)
    }
    #[doc = "GSM mode operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GM_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Base Clock Pulse"]
    #[inline(always)]
    pub fn bcp(&self) -> BCP_R {
        BCP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Parity Mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Block Transfer Mode"]
    #[inline(always)]
    pub fn blk(&self) -> BLK_R {
        BLK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GSM Mode"]
    #[inline(always)]
    pub fn gm(&self) -> GM_R {
        GM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Select"]
    #[inline(always)]
    pub fn cks(&mut self) -> CKS_W<0> {
        CKS_W::new(self)
    }
    #[doc = "Bits 2:3 - Base Clock Pulse"]
    #[inline(always)]
    pub fn bcp(&mut self) -> BCP_W<2> {
        BCP_W::new(self)
    }
    #[doc = "Bit 4 - Parity Mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<4> {
        PM_W::new(self)
    }
    #[doc = "Bit 5 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<5> {
        PE_W::new(self)
    }
    #[doc = "Bit 6 - Block Transfer Mode"]
    #[inline(always)]
    pub fn blk(&mut self) -> BLK_W<6> {
        BLK_W::new(self)
    }
    #[doc = "Bit 7 - GSM Mode"]
    #[inline(always)]
    pub fn gm(&mut self) -> GM_W<7> {
        GM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Serial Mode Register for Smart Card Interface Mode (SCMR.SMIF = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smr_smci](index.html) module"]
pub struct SMR_SMCI_SPEC;
impl crate::RegisterSpec for SMR_SMCI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [smr_smci::R](R) reader structure"]
impl crate::Readable for SMR_SMCI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smr_smci::W](W) writer structure"]
impl crate::Writable for SMR_SMCI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMR_SMCI to value 0"]
impl crate::Resettable for SMR_SMCI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
