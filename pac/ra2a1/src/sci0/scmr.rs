#[doc = "Register `SCMR` reader"]
pub struct R(crate::R<SCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCMR` writer"]
pub struct W(crate::W<SCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCMR_SPEC>;
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
impl From<crate::W<SCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMIF` reader - Smart Card Interface Mode Select"]
pub type SMIF_R = crate::BitReader<SMIF_A>;
#[doc = "Smart Card Interface Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMIF_A {
    #[doc = "0: Non-smart card interface mode(Asynchronous mode, clock synchronous mode, simple SPI mode, or simple I2C mode)"]
    _0 = 0,
    #[doc = "1: Smart card interface mode"]
    _1 = 1,
}
impl From<SMIF_A> for bool {
    #[inline(always)]
    fn from(variant: SMIF_A) -> Self {
        variant as u8 != 0
    }
}
impl SMIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMIF_A {
        match self.bits {
            false => SMIF_A::_0,
            true => SMIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SMIF_A::_1
    }
}
#[doc = "Field `SMIF` writer - Smart Card Interface Mode Select"]
pub type SMIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCMR_SPEC, SMIF_A, O>;
impl<'a, const O: u8> SMIF_W<'a, O> {
    #[doc = "Non-smart card interface mode(Asynchronous mode, clock synchronous mode, simple SPI mode, or simple I2C mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMIF_A::_0)
    }
    #[doc = "Smart card interface mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMIF_A::_1)
    }
}
#[doc = "Field `SINV` reader - Transmitted/Received Data InvertSet this bit to 0 if operation is to be in simple I2C mode."]
pub type SINV_R = crate::BitReader<SINV_A>;
#[doc = "Transmitted/Received Data InvertSet this bit to 0 if operation is to be in simple I2C mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SINV_A {
    #[doc = "0: TDR contents are transmitted as they are. Receive data is stored as it is in RDR."]
    _0 = 0,
    #[doc = "1: TDR contents are inverted before being transmitted. Receive data is stored in inverted form in RDR."]
    _1 = 1,
}
impl From<SINV_A> for bool {
    #[inline(always)]
    fn from(variant: SINV_A) -> Self {
        variant as u8 != 0
    }
}
impl SINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINV_A {
        match self.bits {
            false => SINV_A::_0,
            true => SINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SINV_A::_1
    }
}
#[doc = "Field `SINV` writer - Transmitted/Received Data InvertSet this bit to 0 if operation is to be in simple I2C mode."]
pub type SINV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCMR_SPEC, SINV_A, O>;
impl<'a, const O: u8> SINV_W<'a, O> {
    #[doc = "TDR contents are transmitted as they are. Receive data is stored as it is in RDR."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SINV_A::_0)
    }
    #[doc = "TDR contents are inverted before being transmitted. Receive data is stored in inverted form in RDR."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SINV_A::_1)
    }
}
#[doc = "Field `SDIR` reader - Transmitted/Received Data Transfer DirectionNOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode.Set this bit to 1 if operation is to be in simple I2C mode."]
pub type SDIR_R = crate::BitReader<SDIR_A>;
#[doc = "Transmitted/Received Data Transfer DirectionNOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode.Set this bit to 1 if operation is to be in simple I2C mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIR_A {
    #[doc = "0: Transfer with LSB first"]
    _0 = 0,
    #[doc = "1: Transfer with MSB first"]
    _1 = 1,
}
impl From<SDIR_A> for bool {
    #[inline(always)]
    fn from(variant: SDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIR_A {
        match self.bits {
            false => SDIR_A::_0,
            true => SDIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDIR_A::_1
    }
}
#[doc = "Field `SDIR` writer - Transmitted/Received Data Transfer DirectionNOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode.Set this bit to 1 if operation is to be in simple I2C mode."]
pub type SDIR_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCMR_SPEC, SDIR_A, O>;
impl<'a, const O: u8> SDIR_W<'a, O> {
    #[doc = "Transfer with LSB first"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDIR_A::_0)
    }
    #[doc = "Transfer with MSB first"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDIR_A::_1)
    }
}
#[doc = "Field `CHR1` reader - Character Length 1(Only valid in asynchronous mode)"]
pub type CHR1_R = crate::BitReader<CHR1_A>;
#[doc = "Character Length 1(Only valid in asynchronous mode)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHR1_A {
    #[doc = "0: Transmit/receive in 9-bit data length"]
    _0 = 0,
    #[doc = "1: Transmit/receive in 8-bit data length(SMR.CHR=0) / in 7bit data length(SMR.CHR=1)"]
    _1 = 1,
}
impl From<CHR1_A> for bool {
    #[inline(always)]
    fn from(variant: CHR1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHR1_A {
        match self.bits {
            false => CHR1_A::_0,
            true => CHR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHR1_A::_1
    }
}
#[doc = "Field `CHR1` writer - Character Length 1(Only valid in asynchronous mode)"]
pub type CHR1_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCMR_SPEC, CHR1_A, O>;
impl<'a, const O: u8> CHR1_W<'a, O> {
    #[doc = "Transmit/receive in 9-bit data length"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHR1_A::_0)
    }
    #[doc = "Transmit/receive in 8-bit data length(SMR.CHR=0) / in 7bit data length(SMR.CHR=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHR1_A::_1)
    }
}
#[doc = "Field `BCP2` reader - Base Clock Pulse 2Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\]
bits"]
pub type BCP2_R = crate::BitReader<BCP2_A>;
#[doc = "Base Clock Pulse 2Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\]
bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCP2_A {
    #[doc = "0: S=93(SMR.BCP\\[1:0\\]=00), 128(SMR.BCP\\[1:0\\]=01), 186(SMR.BCP\\[1:0\\]=10), 512(SMR.BCP\\[1:0\\]=11)"]
    _0 = 0,
    #[doc = "1: S=32(SMR.BCP\\[1:0\\]=00), 64(SMR.BCP\\[1:0\\]=01), 372(SMR.BCP\\[1:0\\]=10), 256(SMR.BCP\\[1:0\\]=11)"]
    _1 = 1,
}
impl From<BCP2_A> for bool {
    #[inline(always)]
    fn from(variant: BCP2_A) -> Self {
        variant as u8 != 0
    }
}
impl BCP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCP2_A {
        match self.bits {
            false => BCP2_A::_0,
            true => BCP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCP2_A::_1
    }
}
#[doc = "Field `BCP2` writer - Base Clock Pulse 2Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\]
bits"]
pub type BCP2_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCMR_SPEC, BCP2_A, O>;
impl<'a, const O: u8> BCP2_W<'a, O> {
    #[doc = "S=93(SMR.BCP\\[1:0\\]=00), 128(SMR.BCP\\[1:0\\]=01), 186(SMR.BCP\\[1:0\\]=10), 512(SMR.BCP\\[1:0\\]=11)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCP2_A::_0)
    }
    #[doc = "S=32(SMR.BCP\\[1:0\\]=00), 64(SMR.BCP\\[1:0\\]=01), 372(SMR.BCP\\[1:0\\]=10), 256(SMR.BCP\\[1:0\\]=11)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCP2_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Smart Card Interface Mode Select"]
    #[inline(always)]
    pub fn smif(&self) -> SMIF_R {
        SMIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitted/Received Data InvertSet this bit to 0 if operation is to be in simple I2C mode."]
    #[inline(always)]
    pub fn sinv(&self) -> SINV_R {
        SINV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitted/Received Data Transfer DirectionNOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode.Set this bit to 1 if operation is to be in simple I2C mode."]
    #[inline(always)]
    pub fn sdir(&self) -> SDIR_R {
        SDIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Character Length 1(Only valid in asynchronous mode)"]
    #[inline(always)]
    pub fn chr1(&self) -> CHR1_R {
        CHR1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Base Clock Pulse 2Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\]
bits"]
    #[inline(always)]
    pub fn bcp2(&self) -> BCP2_R {
        BCP2_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Smart Card Interface Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn smif(&mut self) -> SMIF_W<0> {
        SMIF_W::new(self)
    }
    #[doc = "Bit 2 - Transmitted/Received Data InvertSet this bit to 0 if operation is to be in simple I2C mode."]
    #[inline(always)]
    #[must_use]
    pub fn sinv(&mut self) -> SINV_W<2> {
        SINV_W::new(self)
    }
    #[doc = "Bit 3 - Transmitted/Received Data Transfer DirectionNOTE: The setting is invalid and a fixed data length of 8 bits is used in modes other than asynchronous mode.Set this bit to 1 if operation is to be in simple I2C mode."]
    #[inline(always)]
    #[must_use]
    pub fn sdir(&mut self) -> SDIR_W<3> {
        SDIR_W::new(self)
    }
    #[doc = "Bit 4 - Character Length 1(Only valid in asynchronous mode)"]
    #[inline(always)]
    #[must_use]
    pub fn chr1(&mut self) -> CHR1_W<4> {
        CHR1_W::new(self)
    }
    #[doc = "Bit 7 - Base Clock Pulse 2Selects the number of base clock cycles in combination with the SMR.BCP\\[1:0\\]
bits"]
    #[inline(always)]
    #[must_use]
    pub fn bcp2(&mut self) -> BCP2_W<7> {
        BCP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Smart Card Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmr](index.html) module"]
pub struct SCMR_SPEC;
impl crate::RegisterSpec for SCMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scmr::R](R) reader structure"]
impl crate::Readable for SCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scmr::W](W) writer structure"]
impl crate::Writable for SCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCMR to value 0xf2"]
impl crate::Resettable for SCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0xf2;
}
