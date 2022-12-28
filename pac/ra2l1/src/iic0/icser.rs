#[doc = "Register `ICSER` reader"]
pub struct R(crate::R<ICSER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICSER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICSER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICSER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICSER` writer"]
pub struct W(crate::W<ICSER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICSER_SPEC>;
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
impl From<crate::W<ICSER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICSER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR0E` reader - Slave Address Register 0 Enable"]
pub type SAR0E_R = crate::BitReader<SAR0E_A>;
#[doc = "Slave Address Register 0 Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAR0E_A {
    #[doc = "0: Disable slave address in SARL0 and SARU0"]
    _0 = 0,
    #[doc = "1: Enable slave address in SARL0 and SARU0"]
    _1 = 1,
}
impl From<SAR0E_A> for bool {
    #[inline(always)]
    fn from(variant: SAR0E_A) -> Self {
        variant as u8 != 0
    }
}
impl SAR0E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAR0E_A {
        match self.bits {
            false => SAR0E_A::_0,
            true => SAR0E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAR0E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAR0E_A::_1
    }
}
#[doc = "Field `SAR0E` writer - Slave Address Register 0 Enable"]
pub type SAR0E_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICSER_SPEC, SAR0E_A, O>;
impl<'a, const O: u8> SAR0E_W<'a, O> {
    #[doc = "Disable slave address in SARL0 and SARU0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAR0E_A::_0)
    }
    #[doc = "Enable slave address in SARL0 and SARU0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAR0E_A::_1)
    }
}
#[doc = "Field `SAR1E` reader - Slave Address Register 1 Enable"]
pub type SAR1E_R = crate::BitReader<SAR1E_A>;
#[doc = "Slave Address Register 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAR1E_A {
    #[doc = "0: Disable slave address in SARL1 and SARU1"]
    _0 = 0,
    #[doc = "1: Enable slave address in SARL1 and SARU1"]
    _1 = 1,
}
impl From<SAR1E_A> for bool {
    #[inline(always)]
    fn from(variant: SAR1E_A) -> Self {
        variant as u8 != 0
    }
}
impl SAR1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAR1E_A {
        match self.bits {
            false => SAR1E_A::_0,
            true => SAR1E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAR1E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAR1E_A::_1
    }
}
#[doc = "Field `SAR1E` writer - Slave Address Register 1 Enable"]
pub type SAR1E_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICSER_SPEC, SAR1E_A, O>;
impl<'a, const O: u8> SAR1E_W<'a, O> {
    #[doc = "Disable slave address in SARL1 and SARU1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAR1E_A::_0)
    }
    #[doc = "Enable slave address in SARL1 and SARU1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAR1E_A::_1)
    }
}
#[doc = "Field `SAR2E` reader - Slave Address Register 2 Enable"]
pub type SAR2E_R = crate::BitReader<SAR2E_A>;
#[doc = "Slave Address Register 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAR2E_A {
    #[doc = "0: Disable slave address in SARL2 and SARU2"]
    _0 = 0,
    #[doc = "1: Enable slave address in SARL2 and SARU2"]
    _1 = 1,
}
impl From<SAR2E_A> for bool {
    #[inline(always)]
    fn from(variant: SAR2E_A) -> Self {
        variant as u8 != 0
    }
}
impl SAR2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAR2E_A {
        match self.bits {
            false => SAR2E_A::_0,
            true => SAR2E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAR2E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAR2E_A::_1
    }
}
#[doc = "Field `SAR2E` writer - Slave Address Register 2 Enable"]
pub type SAR2E_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICSER_SPEC, SAR2E_A, O>;
impl<'a, const O: u8> SAR2E_W<'a, O> {
    #[doc = "Disable slave address in SARL2 and SARU2"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAR2E_A::_0)
    }
    #[doc = "Enable slave address in SARL2 and SARU2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAR2E_A::_1)
    }
}
#[doc = "Field `GCAE` reader - General Call Address Enable"]
pub type GCAE_R = crate::BitReader<GCAE_A>;
#[doc = "General Call Address Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCAE_A {
    #[doc = "0: Disable general call address detection"]
    _0 = 0,
    #[doc = "1: Enable general call address detection"]
    _1 = 1,
}
impl From<GCAE_A> for bool {
    #[inline(always)]
    fn from(variant: GCAE_A) -> Self {
        variant as u8 != 0
    }
}
impl GCAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCAE_A {
        match self.bits {
            false => GCAE_A::_0,
            true => GCAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GCAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GCAE_A::_1
    }
}
#[doc = "Field `GCAE` writer - General Call Address Enable"]
pub type GCAE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICSER_SPEC, GCAE_A, O>;
impl<'a, const O: u8> GCAE_W<'a, O> {
    #[doc = "Disable general call address detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GCAE_A::_0)
    }
    #[doc = "Enable general call address detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GCAE_A::_1)
    }
}
#[doc = "Field `DIDE` reader - Device-ID Address Detection Enable"]
pub type DIDE_R = crate::BitReader<DIDE_A>;
#[doc = "Device-ID Address Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIDE_A {
    #[doc = "0: Disable device-ID address detection"]
    _0 = 0,
    #[doc = "1: Enable device-ID address detection"]
    _1 = 1,
}
impl From<DIDE_A> for bool {
    #[inline(always)]
    fn from(variant: DIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl DIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIDE_A {
        match self.bits {
            false => DIDE_A::_0,
            true => DIDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIDE_A::_1
    }
}
#[doc = "Field `DIDE` writer - Device-ID Address Detection Enable"]
pub type DIDE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICSER_SPEC, DIDE_A, O>;
impl<'a, const O: u8> DIDE_W<'a, O> {
    #[doc = "Disable device-ID address detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIDE_A::_0)
    }
    #[doc = "Enable device-ID address detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIDE_A::_1)
    }
}
#[doc = "Field `HOAE` reader - Host Address Enable"]
pub type HOAE_R = crate::BitReader<HOAE_A>;
#[doc = "Host Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HOAE_A {
    #[doc = "0: Disable host address detection"]
    _0 = 0,
    #[doc = "1: Enable host address detection"]
    _1 = 1,
}
impl From<HOAE_A> for bool {
    #[inline(always)]
    fn from(variant: HOAE_A) -> Self {
        variant as u8 != 0
    }
}
impl HOAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOAE_A {
        match self.bits {
            false => HOAE_A::_0,
            true => HOAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HOAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HOAE_A::_1
    }
}
#[doc = "Field `HOAE` writer - Host Address Enable"]
pub type HOAE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICSER_SPEC, HOAE_A, O>;
impl<'a, const O: u8> HOAE_W<'a, O> {
    #[doc = "Disable host address detection"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HOAE_A::_0)
    }
    #[doc = "Enable host address detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HOAE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Address Register 0 Enable"]
    #[inline(always)]
    pub fn sar0e(&self) -> SAR0E_R {
        SAR0E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Address Register 1 Enable"]
    #[inline(always)]
    pub fn sar1e(&self) -> SAR1E_R {
        SAR1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Address Register 2 Enable"]
    #[inline(always)]
    pub fn sar2e(&self) -> SAR2E_R {
        SAR2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - General Call Address Enable"]
    #[inline(always)]
    pub fn gcae(&self) -> GCAE_R {
        GCAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Device-ID Address Detection Enable"]
    #[inline(always)]
    pub fn dide(&self) -> DIDE_R {
        DIDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Host Address Enable"]
    #[inline(always)]
    pub fn hoae(&self) -> HOAE_R {
        HOAE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Address Register 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sar0e(&mut self) -> SAR0E_W<0> {
        SAR0E_W::new(self)
    }
    #[doc = "Bit 1 - Slave Address Register 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sar1e(&mut self) -> SAR1E_W<1> {
        SAR1E_W::new(self)
    }
    #[doc = "Bit 2 - Slave Address Register 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sar2e(&mut self) -> SAR2E_W<2> {
        SAR2E_W::new(self)
    }
    #[doc = "Bit 3 - General Call Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcae(&mut self) -> GCAE_W<3> {
        GCAE_W::new(self)
    }
    #[doc = "Bit 5 - Device-ID Address Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dide(&mut self) -> DIDE_W<5> {
        DIDE_W::new(self)
    }
    #[doc = "Bit 7 - Host Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hoae(&mut self) -> HOAE_W<7> {
        HOAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icser](index.html) module"]
pub struct ICSER_SPEC;
impl crate::RegisterSpec for ICSER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [icser::R](R) reader structure"]
impl crate::Readable for ICSER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icser::W](W) writer structure"]
impl crate::Writable for ICSER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICSER to value 0x09"]
impl crate::Resettable for ICSER_SPEC {
    const RESET_VALUE: Self::Ux = 0x09;
}
